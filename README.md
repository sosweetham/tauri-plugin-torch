# Tauri Plugin Torch

This project is a Tauri plugin which allows for flash control on iOS and Android devices with a consistent API.

On Android, turning on the torch while another component holds the camera
normally fails. This plugin also works while a
[barcode scanner](#barcode-scanner) scan is running, so apps can offer a
flashlight button in their scanning UI.

| Platform | Supported |
| -------- | --------- |
| Linux    | x         |
| Windows  | x         |
| macOS    | x         |
| Android  | ✓         |
| iOS      | ✓         |

## Install

Add the Rust crate to your app:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-torch = "0.1"
# alternatively with Git:
tauri-plugin-torch = { git = "https://github.com/sosweetham/tauri-plugin-torch" }
```

And the JavaScript guest bindings:

```sh
pnpm add @sosweetham/tauri-plugin-torch-api
# or
npm add @sosweetham/tauri-plugin-torch-api
# or
yarn add @sosweetham/tauri-plugin-torch-api
```

## Usage

Register the plugin with Tauri:

`src-tauri/src/lib.rs`

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_torch::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Then allow its commands in your capability:

`src-tauri/capabilities/default.json`

```json
{
    "permissions": ["core:default", "torch:default"]
}
```

## API

### Available Commands

```ts
import {
    disable,
    enable,
    isAvailable,
    isEnabled,
    toggle,
} from "@sosweetham/tauri-plugin-torch-api";

if (await isAvailable()) {
    await enable();
    console.log(await isEnabled()); // true
    await toggle(); // returns false, the torch is now off
    await disable();
}
```

`enable`, `disable` and `toggle()` reject when the device has no torch, the
torch is temporarily unavailable (e.g. the device is overheating) or the camera
is held by another app. On desktop, `isAvailable` and `isEnabled` return
`false` and `enable`, `disable` and `toggle()` reject.

`toggle(value)` and `check()` from earlier versions still work but are
deprecated: use `enable`/`disable` and `isEnabled` instead. As before, they
resolve to `null` instead of rejecting when the device has no torch.

`isEnabled` reads the hardware state, so it also reflects the torch being
turned off by the system or another app.

On iOS, apps can't see the torch state set by Control Center or another app:
while the flashlight is on from there, `isEnabled` returns `false` and `toggle`
tries to turn the torch on.

### Barcode scanner

The torch can be controlled while a
[tauri-plugin-barcode-scanner](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/barcode-scanner)
scan is running, e.g. to add a flashlight button to the scanning UI so codes
can be read in the dark.

On Android the scanner holds the camera during a scan, and the system rejects
regular torch requests while it does. This plugin detects an active scan and
drives the torch through the scanner's camera instead, which needs
`tauri-plugin-barcode-scanner` **2.5.3 or newer**. With older versions,
`enable`, `disable` and `toggle()` reject with "the camera is in use by
another app" while scanning. iOS has no such restriction and works with any
version.

1. Install and register both plugins. The barcode scanner only builds for
   Android and iOS, and the torch only does something there, so add both as
   mobile-only dependencies (instead of the regular `[dependencies]` entry from
   [Install](#install)):

    `src-tauri/Cargo.toml`

    ```toml
    [target.'cfg(any(target_os = "android", target_os = "ios"))'.dependencies]
    tauri-plugin-barcode-scanner = "2.5.3"
    tauri-plugin-torch = "0.1"
    ```

    Then register them on the mobile-only branch of your builder:

    `src-tauri/src/lib.rs`

    ```rust
    #[cfg_attr(mobile, tauri::mobile_entry_point)]
    pub fn run() {
        let builder = tauri::Builder::default();

        #[cfg(mobile)]
        let builder = builder
            .plugin(tauri_plugin_barcode_scanner::init())
            .plugin(tauri_plugin_torch::init());

        builder
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    ```

    And install both guest bindings:

    ```sh
    pnpm add @tauri-apps/plugin-barcode-scanner @sosweetham/tauri-plugin-torch-api
    ```

    If you also need the torch commands on desktop (where they report the torch
    as off), keep `tauri-plugin-torch` in the regular `[dependencies]` and
    register it outside the `#[cfg(mobile)]` branch instead.

2. Allow both plugins' commands. Neither is registered on desktop, so enable
   them in a mobile-only capability (and leave `torch:default` out of your
   desktop capability):

    `src-tauri/capabilities/mobile.json`

    ```json
    {
        "identifier": "mobile",
        "platforms": ["android", "iOS"],
        "windows": ["main"],
        "permissions": ["barcode-scanner:default", "torch:default"]
    }
    ```

3. Scan with `windowed: true` so your webview UI, including the torch button,
   stays visible on top of the camera preview (the scanner makes the webview
   transparent; see the barcode scanner docs for the styling this needs), and
   call the torch API while the scan is pending:

    ```ts
    import {
        cancel,
        Format,
        requestPermissions,
        scan,
    } from "@tauri-apps/plugin-barcode-scanner";
    import { isAvailable, toggle } from "@sosweetham/tauri-plugin-torch-api";

    async function startScan() {
        if ((await requestPermissions()) !== "granted") return;

        // show the torch button only on devices that have a torch
        torchButton.hidden = !(await isAvailable());

        try {
            const result = await scan({ windowed: true, formats: [Format.QRCode] });
            console.log(result.content);
        } finally {
            torchButton.hidden = true;
        }
    }

    torchButton.onclick = async () => {
        const on = await toggle();
        torchButton.textContent = on ? "Torch off" : "Torch on";
    };

    cancelButton.onclick = () => cancel();
    ```

The torch goes off together with the camera when the scan finishes or is
cancelled. `isEnabled` reads the hardware state, so it reports that too; use it
instead of tracking the state yourself if your UI outlives the scan. To turn
the torch off before the scan ends, call `disable()`.

The
[plugins-workspace API example](https://github.com/tauri-apps/plugins-workspace/blob/v2/examples/api/src/views/Scanner.svelte)
shows a full scanner UI with a torch button.

### Rust

The Rust API is available through the `TorchExt` trait:

```rust
use tauri_plugin_torch::TorchExt;

if app.torch().is_available()? {
    app.torch().enable()?;
}
```

### Default Permission

This permission set configures which
torch features are by default exposed.

#### Granted Permissions

It allows access to all torch commands.

#### This default permission set includes the following:

-   `allow-check`
-   `allow-disable`
-   `allow-enable`
-   `allow-is-available`
-   `allow-is-enabled`
-   `allow-toggle`

### Permission Table

<table>
<tr>
<th>Identifier</th>
<th>Description</th>
</tr>


<tr>
<td>

`torch:allow-check`

</td>
<td>

Enables the check command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:deny-check`

</td>
<td>

Denies the check command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:allow-disable`

</td>
<td>

Enables the disable command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:deny-disable`

</td>
<td>

Denies the disable command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:allow-enable`

</td>
<td>

Enables the enable command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:deny-enable`

</td>
<td>

Denies the enable command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:allow-is-available`

</td>
<td>

Enables the is_available command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:deny-is-available`

</td>
<td>

Denies the is_available command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:allow-is-enabled`

</td>
<td>

Enables the is_enabled command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:deny-is-enabled`

</td>
<td>

Denies the is_enabled command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:allow-toggle`

</td>
<td>

Enables the toggle command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`torch:deny-toggle`

</td>
<td>

Denies the toggle command without any pre-configured scope.

</td>
</tr>
</table>

# Contribution

## Notes

1. Be sure to deploy the app using `pnpm tauri [android|ios] dev` to get the best experience. The IDEs are not aware of the Tauri build process and will not work as expected.
2. Do it beforehand development because otherwise tauri packages will not be generated for the swift and kotlin code.

### Development Troubleshooting

-   Be sure to open the `android` and `ios` folders in their respective IDEs (Android Studio and XCode) to get the best development experience.
-   Do not deploy the app using the buttons provided in the IDEs, use the Tauri CLI instead. The IDEs are not aware of the Tauri build process and will not work as expected.
-   You must build the plugin and guest-js before deploying the app using `cargo build` and `pnpm build` respectively.
-   The tauri imports may give import errors in the IDEs if you dont use the plugin and deploy the app first.

## Looking for

-   Automated Testing Suite - Implementation for testing suite of the plugin would be highly appreciated
-   Documentation - Doc comments are lackluster rn for a library project

## Physical Testing

I have tested the plugin on a Pixel 8 Pro (Android 15) and iPhone 13 (iOS 18) and it works as expected. The implementations are really generic and should work on all devices. If you have a device that is not working, please open an issue.
