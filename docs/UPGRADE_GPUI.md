# GPUI Upgrade Checklist

1. 建立獨立升級分支。
2. 更新根目錄所有 GPUI-family dependency 到相同 commit。
3. 執行 `cargo update` 並檢查 lockfile 來源一致。
4. 修正 `Render`、`RenderOnce`、`Context`、`Window`、style builder 與 callback API。
5. 執行 workspace fmt/check/clippy/test。
6. 啟動 Gallery 驗證 Theme、Button、Switch、Progress。
7. 在 Windows、Linux、macOS 至少各完成一次 check。
8. 在此文件新增日期、舊 revision、新 revision、破壞變更與修正摘要。
