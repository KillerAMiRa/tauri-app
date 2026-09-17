# Tauri + URL 

本项目是一套基于`网址（URL）`打包原生 APP 的云端编译方案，核心特性：
- 服务端驱动，`免升级发版`：APP 内容由远端网址承载，业务更新只需发布服务器，终端无需发版、用户无感升级。
- 灵活定制包名：一套代码可编译出`不同应用包名`（Application ID / Bundle ID），支持多客户、多渠道独立打包。
- 双 `WebView` 长效保活：采用主备双 WebView 切换机制，配合定时刷新（默认 `30 分钟`）回收内存，杜绝内存泄漏与页面卡顿，可长期稳定运行，对大屏展示、数字标牌、展厅等 `7×24` 小时在线场景非常友好。

## Recommended IDE Setup

- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) 
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

[Tauri文档](https://v2.tauri.app/zh-cn/distribute/sign/android/)

[英文文档](https://v2.tauri.app/distribute/sign/android/)

[签名](https://tool.lvtao.net/keystore)

[包分析](https://tool.tds.qq.com/apk-analyzer)
```
pnpm tauri icon

pnpm tauri android build 打默认包（包名：com.chens.lumina_t，名字：Lumina_t）
# 自定义包名和名字
$env:CUSTOM_APP_ID="com.chens.lumina.ranking"; $env:CUSTOM_APP_NAME="ranking_L"; pnpm tauri android build

pnpm tauri android dev
pnpm tauri android init
```
Lumina_t_s

http manifestPlaceholders["usesCleartextTraffic"] = "true"
