# Developing SwiftUI within nvim

Here are few thoughts and observations on SwiftUI development outside xcode.

First of all I did setup [XVim2] and yes, I tottaly mimiced my nvim shortcuts
with xvimrc, but still I couldn't do it. Indeed, Xcode is magnificent and one
of top noatch IDE, if not the best for apple software development. However, it
is constraining and limiting in so many ways, in addition to UI delays and
mouse driven workflow _"Everything is few Clicks away"_, WELL ... my mouse usually
lost somewhere and I'd have top stop what I'm working on, move my hands of the keyboard, locate the mouse and
"Click Away" .... 🙄

The only thing I end up missing is `XCodePreviews` for SwiftUI and hopefully at
the end of ths experiments and research I will enup with an alternative.

[XVim2]: https://github.com/XVimProject/XVim2

## `xcodebuild` command line tool

No need to reinvent the wheal or manually do `swiftc`, `clang`, linking
frameworks .. etc,  just use `xcodebuild`. However, there is a catch (always)
😄. You would need `.xcodeproj` for `xcodebuild` to even consider your project.

Sadly `.xcodeproj` isn't a file you can edit by hand or script. It's something
Xcode generate and maintain in order to function and process projects. If you
do anything outside xcode like adding files, xcode will not recognize those
files. Luckily, there is an awesome generator that generate `.xcodeproj` called [XcodeGen]

Basically, you define `project.yml` which acts like `Cargo.toml`,
`package.json` or `deps.edn` and then run `xcodegen generate` to get your
`.xcodeproj` without opening xcode. Check out [ProjectSpec] and [Usage] to
understand how to define your `project.yml`

[XcodeGen]: https://github.com/yonaskolb/XcodeGen
[ProjectSpec]: https://github.com/yonaskolb/XcodeGen/blob/master/Docs/ProjectSpec.md
[Usage]: https://github.com/yonaskolb/XcodeGen/blob/master/Docs/Usage.md

### Issues

1. Every time a new file is added or renamed, `xcodegen generate` need to be re-ran.
2. If you have xcode open, you need to close it and repon it after generating.

### Tips

1. use environment variables instead of hardcoding variables that you want to keep out of project.yaml or is auto generated.
  set a key to `${SOMENVVAL}` then export `SOMENVVAL=1`

### Project.yml
Here a fully working ios/SwiftUI project.yml

~~~yml
name: Demo
options:
  bundleIdPrefix: tami5
  preGenCommand: killall Xcode || true
targets:
  Demo:
    type: application
    platform: iOS
    deploymentTarget: 15.0
    sources: [ Sources, Resources ]
    info:
      path: Resources/Info.plist
      properties:
        CFBundleShortVersionString: "1.0"
        UILaunchStoryboardName: LaunchScreen
        UIRequiresFullScreen: true
        UISupportedInterfaceOrientations~iphone:
          - UIInterfaceOrientationPortrait
    settings:
      DISABLE_MANUAL_TARGET_ORDER_BUILD_WARNING: 'YES'
    preBuildScripts:
      - name: Format Swift Files
        script: swiftformat . --swiftversion 5.5
      - name: Increment CFBundleVersion
        script: /usr/libexec/PlistBuddy -c "Set :CFBundleVersion $(date -u "+%g%m%d")" Resources/Info.plist
~~~


## Integration with Simulator.app

Here what I end-up making to integrate and run my app on Simulator.app. In
neovim, I used [asyncrun.nvim] to run make simboot and then simrun.

### Issues:

1. Getting the workflow right, currently I have three make commands to manage this as outline above
2. Waiting for Simulator.app to run after closing it by mistake.
3. Slight delay when opening new booting a simulator and Simulator.app

> On a side note, maybe using [ios-sim] would fix some of the aforementioned issues.

### Ideas:

1. Attach a watch process when simrun get called that would rebuild and
   reinstall the app on file changes.
2. Neovim Integration: Nested Picker with Run choice that would expand to have
   all the supported/installed simulators.
    - After first run, a `g:` variable need to be used in status plugins to
      indicate to the user that there is a process running.
    - Either a dedicate watch process or on BufWrite to trigger rebuild and
      reinstall to the simulator. From a techincal aspect, many other processes
      might require to react to file changes, so perhaps there would be a
      central function that gets called to call all registred callbacks for file changes.
    - Toggle logs buffer for current process, this should be as command in the main/nested picker.
    - As part of rebuilding, if a new file is added, or project.yml is edited,
      the project should be cleaned and regeneratd.
3. Print statement doesn't show and requires os_log to be used.

[ios-sim]: https://github.com/ios-control/ios-sim
[asyncrun.nvim]: https://github.com/skywind3000/asyncrun.vim

### Makefile:

~~~make
PNAME=Demo
PID=tami5.$(PNAME)
PSCHEME=-scheme $(PNAME)

SIMDEVICE="iPhone\ 13"
SIMDEBUG=log stream --level debug --style compact --predicate 'subsystem == "$(PID)"'

XTARGET=-target $(PNAME)
XARCHIVE=-project $(PNAME).xcodeproj -archivePath $(PNAME).xcarchive archive
XARCHIVEAPP="./$(PNAME).xcarchive/Products/Applications/$(PNAME).app"
XBUILDARGS=$(XTARGET)

simbuild:
	xcodebuild $(PSCHEME) $(XPROJECT) $(XARCHIVE) -sdk iphonesimulator  | xcbeautify

simboot:
	xcrun simctl shutdown booted
	xcrun simctl boot "$(SIMNAME)"

simrun: simbuild
	xcrun simctl install "$(SIMDEVICE)" $(XARCHIVEAPP)
	xcrun simctl launch booted $(PID)
	open -a Simulator.app
	xcrun simctl spawn booted $(SIMDEBUG)
~~~


## Deploying to a physical device

Running the SwiftUI app on an acutal device. This is the second feature I
wanted to have. It's important to play with the app on actual device.
Luckily, [ios-deploy] came to rescue. After installing with brew, it worked out
of the box without any side-hussle.

Sadly, unlike, running on xcode, and due to my current workflow, the process
isn't as straightforward.

In Xcode, you would just do `CMD-R` to relunch the app and expect everything to
work flawlessly. For me with [ios-deploy] the process start out by running
`make run` then wait some time for build to finish then hope it doesn't crash.
Then, make changes `<C-c>`  and re-execute `make run`.

### Issues

1. If the app errors I need to unplagued and replugged to get it working again.
2. Sometimes the device lags between deployments, and similarly require unplugging and replugging.
3. Huge number of leaking ios-deploy processes between rebuilds, perhaps due to my misuse.

### Ideas

1. Maybe there is a way for ios-deploy to re-deplay automatically and be faster?
2. Neovim Integration: With the nested picker, run section, detect if a device
   is connected and show it as open for Run picker.
   - Similarly to simulator process, rebuild and re-deploy on file changes.

### Makefile

~~~make
build:
	echo "⚙️ Building ..."
	set -o pipefail && xcodebuild | tee build.log | xcbeautify

deploy: build
	echo "⚙️ Deploying to connected device ...."
	ios-deploy --debug --bundle build/Debug-iphoneos/$(PNAME).app
~~~


[ios-deploy]: https://github.com/ios-control/ios-deploy

## Integrating [sourcekit-lsp]

Using [sourcekit-lsp] isn't that hard with [nvim-lspconfig]. However, getting
completion and jump to code location is harder and require user intervention.
After developing for sometime without these features, I discovered
[xcode-build-server].

Basically, I need to have xcodebuild logs and redirect it to
`xcode-build-server parse` which produced `buildServer.json` and `.compile` and
thats it code completion and code jump started working.

Though, when the build environment changes (eg: add new files, switch sdk,
debug/release, conditional macro, etc..), The previous step needs to repeated.
In addition, if the build directory doesn't exist, the server will stop
working.

### Issues

1. Requires clean build.
2. Populate current directory with `buildAServer.json` and `.compile` which need to be ignored.

### Scirpt

```make

# Must run every time a file is added or renamed. As well as when project.yml get edited Not sure when files changes right now
all: clean xcodeproj build sourcekit

clean:
	echo "⚙️ Cleaning up build files ..."
	xcodebuild clean &> /dev/null

xcodeproj:
	echo "⚙️ Generating xcodeproj ..."
	xcodegen generate &> /dev/null

build:
	echo "⚙️ Building ..."
	xcodebuild | tee build.log | xcbeautify

sourcekit:
	echo "⚙️ Generating compiled commands ..."
	cat build.log | xcode-build-server parse && rm build.log
```

[nvim-lspconfig]: https://github.com/neovim/nvim-lspconfig
[sourcekit-lsp]: https://github.com/apple/sourcekit-lsp
[xcode-build-server]: https://github.com/SolaWing/xcode-build-server


## Code Injection, Hot reload and preview changes:

rebuilds takes some time and UI iteration takes longer due to this issue. I
tried to find alternatives found few open source utilities that might help
with this issues.

[HotReloading]: tried it briefly and not 100% sure I set it up correctly as it
requires a setup phase. It worked for the simulator okay but didn't work with
the testing device due to some idk, local ports to be opened. I noticed when
making changes in one page, while viewing another page, the simulator would
return to the page I just save. Although, more testing required,

[HotReloading]: https://github.com/johnno1962/HotReloading

## SwiftUI observations

### Code duplication

Some logic and code better kept either as a reference or to copy and
slightly modify. These are things like as animations or extensions.

### Filenames

At first I wanted to go with snake_case, then understood how Pasical Cases is
better in Apple development. One reason is, unique filenames?, maybe there is a
workaround this one, but you can't have to files share the same name even if
they are located in different directory. so `./views/root.swift` and
`./models/root.swift` won't work

Additionally, every class/struct/view should have their own file with identical
name in a directory or group that matches their supper type

## Next

Build an IDE like [vscode-SDE], [flutter-tools] and [vscode-swift] to make
development experience fast and straightforward.

- On new file, delete file, restart lsp server.
- Generate function documentation comment based on the function declaration, triggered by pressing "/"
- Display build/running status in statusline.
- Command Picker
- Standalone watch process / socket to receive request for all nvim editors
- Strip .swift from names and relay on icons in bufferline and telescope picker

[vscode-SDE]: https://github.com/vknabel/vscode-swift-development-environment#using-sourcekit-lsp
[vscode-swift]: https://github.com/swift-server/vscode-swift
[flutter-tools]: https://github.com/akinsho/flutter-tools.nvim
