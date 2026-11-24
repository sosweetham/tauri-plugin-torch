# Tauri Plugin Torch

This project is a Tauri plugin which allows for flash control on iOS and Android devices with a consistent API.

| Platform | Supported |
| -------- | --------- |
| Linux    | x         |
| Windows  | x         |
| macOS    | x         |
| Android  | ✓         |
| iOS      | ✓         |

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

On Android, turning on the torch while another component holds the camera
normally fails. This plugin also works while a
[tauri-plugin-barcode-scanner](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/barcode-scanner)
scan is running, so apps can offer a flashlight button in their scanning UI.
This needs `tauri-plugin-barcode-scanner` 2.5.3 or newer.

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
