<p align="center">
<a href="./src/icons/icon.svg">
<img src="./src/icons/icon.svg" width="100" height="100" alt="icon">
</a>
<h1 align="center">Mobile Mouse Server</h1>
</p>

> The server for [Mobile Mouse](https://github.com/iewnfod/MobileMouse)

## FAQs
### "Mobile Mouse Server" has damaged, you should move it to trash
Run the following script in your terminal.
```shell
sudo spctl --master-disable
sudo xattr -rd com.apple.quarantine /Applications/Mobile\ Mouse\ Server.app
```

### When you cannot activate the window automatically after pressed function key...
The permission has already been added to TCC database.
You should manually activate it by following steps:
1. Open `System Settings`.
2. Enter `Privacy and Security`.
3. Select `Accessibility`.
4. Turn on the switch for `Mobile Mouse Server`.
