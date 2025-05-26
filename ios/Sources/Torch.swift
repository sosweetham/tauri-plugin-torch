import SwiftRs
import Tauri
import UIKit
import WebKit
import AVFoundation

class PingArgs: Decodable {
  let value: String?
}

class ToggleArgs: Decodable {
  let value: Bool?
}

class TorchPlugin: Plugin {
  var isOn = false

  @objc public func toggle(_ invoke: Invoke) throws {
    let turnOn = try invoke.parseArgs(ToggleArgs.self).value ?? false

    guard let device = AVCaptureDevice.default(for: .video),
          device.hasTorch else {
      invoke.resolve(["value": false])
      return
    }

    do {
      try device.lockForConfiguration()
      if turnOn {
        try device.setTorchModeOn(level: 1.0)
        self.isOn = true
      } else {
        device.torchMode = .off
        self.isOn = false
      }
      device.unlockForConfiguration()
      invoke.resolve(["value": turnOn])
    } catch {
      self.isOn = false
      invoke.reject("Failed to access torch: \(error.localizedDescription)")
    }
  }

  @objc public func check(_ invoke: Invoke) throws {
    invoke.resolve(["value": self.isOn])
  }
}

@_cdecl("init_plugin_torch")
func initPlugin() -> Plugin {
  return TorchPlugin()
}
