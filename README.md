# Obsidian-OS Compositor

A compositor is a piece of software which routes inputs to windows, and displays window contens on a screen. This is achieved using a Client/Server model which follows the [Wayland](https://wayland.freedesktop.org/) protocol. A compositor is a program which implements Wayland.

This compositor is special because it implements a secondary protocol to communicate with an Obsidian plugin. This plugin handles events to/from the compositor in order to display wayland clients as tabs in Obsidian, allowing for seamless usage of other apps, while taking advantage of Obsidian's workspace features.
The secondary protocol, internally referred to as delegated wayland allows applications to report a state via a URL, which can be used to resume sessions or open apps with specific resources. Examples may include Obsidian itself, Spotify or any other app using URLs.

Obsidian is the only client which will run on the DRM interface implemented by the compositor, as it should serve as the desktop environment. The compositor implements the mechanisms for mutli-window handling, which in in this circumstance corresponds to an external display, and ,manages interactions between both windows. 

## Cargo

The compositor is built upon [smithay](https://wayland.freedesktop.org/) and is a fork of the [Smallvil](https://github.com/Smithay/smithay/tree/master/smallvil) example compositor using Winit. It requires the following features:
* `desktop`
* `backend_winit`
* `wayland_frontend`
* `backend_session`
* `backend_session_libseat`
* `backend_udev`
* `backend_libinput`
* `backend_drm`

These are each depended upon automatically, and you shouldn't need to do anything. They are here for reference.
