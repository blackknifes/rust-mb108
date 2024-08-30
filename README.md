# rust-mb108
rust-mb108

## 目前mb108原生api实现清单: 
| api               | 是否实现 |
|-------------------|-------|
| mbUninit | √ |
| mbCreateInitSettings | √ |
| mbSetInitSettings | √ |
| mbCreateWebView | √ |
| mbCreateWebViewBindGtkWindow | × |
| mbDestroyWebView | √ |
| mbCreateWebWindow | √ |
| mbCreateWebCustomWindow | × |
| mbMoveWindow | × |
| mbMoveToCenter | √ |
| mbSetAutoDrawToHwnd | × |
| mbGetCaretRect | × |
| mbSetAudioMuted | × |
| mbIsAudioMuted | × |
| mbCreateString | √ |
| mbCreateStringWithout×Termination | × |
| mbDeleteString | √ |
| mbGetStringLen | √ |
| mbGetString | √ |
| mbSetProxy | × |
| mbSetDebugConfig | √ |
| mbNetSetData | √ |
| mbNetHookRequest | × |
| mbNetChangeRequestUrl | √ |
| mbNetContinueJob | × |
| mbNetGetRawHttpHeadInBlinkThread | × |
| mbNetGetRawResponseHeadInBlinkThread | × |
| mbNetHoldJobToAsynCommit | × |
| mbNetCancelRequest | √ |
| mbNetOnResponse | × |
| mbNetSetWebsocketCallback | × |
| mbNetSendWsText | × |
| mbNetSendWsBlob | × |
| mbNetEnableResPacket | × |
| mbNetGetPostBody | × |
| mbNetCreatePostBodyElements | × |
| mbNetFreePostBodyElements | × |
| mbNetCreatePostBodyElement | × |
| mbNetFreePostBodyElement | × |
| mbNetCreateWebUrlRequest | × |
| mbNetAddHTTPHeaderFieldToUrlRequest | × |
| mbNetStartUrlRequest | × |
| mbNetGetHttpStatusCode | × |
| mbNetGetRequestMethod | × |
| mbNetGetExpectedContentLength | × |
| mbNetGetResponseUrl | × |
| mbNetCancelWebUrlRequest | × |
| mbSetViewProxy | × |
| mbNetSetMIMEType | √ |
| mbNetGetMIMEType | × |
| mbNetGetHTTPHeaderField | × |
| mbNetSetHTTPHeaderField | × |
| mbNetSetHTTPHeaderFieldUtf8 | √ |
| mbSetMouseEnabled | × |
| mbSetTouchEnabled | × |
| mbSetSystemTouchEnabled | × |
| mbSetContextMenuEnabled | × |
| mbSetNavigationToNewWindowEnable | √ |
| mbSetHeadlessEnabled | × |
| mbSetDragDropEnable | × |
| mbSetDragEnable | × |
| mbSetContextMenuItemShow | × |
| mbSetHandle | √ |
| mbSetHandleOffset | × |
| mbGetPlatformWindowHandle | × |
| mbGetHostHWND | √ |
| mbSetTransparent | × |
| mbSetViewSettings | × |
| mbSetCspCheckEnable | √ |
| mbSetNpapiPluginsEnabled | × |
| mbSetMemoryCacheEnable | × |
| mbSetCookie | × |
| mbSetCookieEnabled | √ |
| mbSetCookieJarPath | × |
| mbSetCookieJarFullPath | × |
| mbSetLocalStorageFullPath | × |
| mbGetTitle | × |
| mbSetWindowTitle | √ |
| mbSetWindowTitleW | × |
| mbGetUrl | × |
| mbGetCursorInfoType | × |
| mbAddPluginDirectory | × |
| mbSetUserAgent | × |
| mbSetZoomFactor | × |
| mbGetZoomFactor | × |
| mbSetDiskCacheEnabled | × |
| mbSetDiskCachePath | × |
| mbSetDiskCacheLimit | × |
| mbSetDiskCacheLimitDisk | × |
| mbSetDiskCacheLevel | × |
| mbSetResourceGc | √ |
| mbCanGoForward | × |
| mbCanGoBack | × |
| mbGetCookie | × |
| mbGetCookieOnBlinkThread | × |
| mbClearCookie | × |
| mbResize | √ |
| mbGetSize | √ |
| mbOnNavigation | √ |
| mbOnNavigationSync | × |
| mbOnCreateView | √ |
| mbOnDocumentReady | √ |
| mbOnPaintUpdated | √ |
| mbOnPaintBitUpdated | × |
| mbOnAcceleratedPaint | × |
| mbOnLoadUrlBegin | √ |
| mbOnLoadUrlEnd | √ |
| mbOnLoadUrlFail | × |
| mbOnTitleChanged | √ |
| mbOnURLChanged | √ |
| mbOnLoadingFinish | √ |
| mbOnDownload | √ |
| mbOnDownloadInBlinkThread | √ |
| mbOnAlertBox | √ |
| mbOnConfirmBox | √ |
| mbOnPromptBox | √ |
| mbOnNetGetFavicon | × |
| mbOnConsole | √ |
| mbOnClose | √ |
| mbOnDestroy | × |
| mbOnPrinting | √ |
| mbOnDidCreateScriptContext | √ |
| mbOnPluginList | × |
| mbOnImageBufferToDataURL | √ |
| mbGoBack | √ |
| mbGoForward | √ |
| mbNavigateAtIndex | × |
| mbGetNavigateIndex | × |
| mbStopLoading | × |
| mbReload | √ |
| mbPerformCookieCommand | × |
| mbEditorSelectAll | × |
| mbEditorCopy | × |
| mbEditorCut | × |
| mbEditorPaste | × |
| mbEditorDelete | × |
| mbEditorUndo | × |
| mbFireMouseEvent | √ |
| mbFireContextMenuEvent | × |
| mbFireMouseWheelEvent | √ |
| mbFireKeyUpEvent | √ |
| mbFireKeyDownEvent | √ |
| mbFireKeyPressEvent | √ |
| mbFireWindowsMessage | √ |
| mbSetFocus | √ |
| mbKillFocus | √ |
| mbShowWindow | √ |
| mbLoadURL | √ |
| mbLoadHtmlWithBaseUrl | × |
| mbPostURL | × |
| mbGetLockedViewDC | √ |
| mbUnlockViewDC | √ |
| mbWake | √ |
| mbJsToDouble | √ |
| mbJsToBoolean | √ |
| mbJsToString | √ |
| mbGetJsValueType | × |
| mbOnJsQuery | √ |
| mbResponseQuery | √ |
| mbRunJs | √ |
| mbRunJsSync | × |
| mbWebFrameGetMainFrame | √ |
| mbIsMainFrame | √ |
| mbSetNodeJsEnable | × |
| mbSetDeviceParameter | × |
| mbGetContentAsMarkup | × |
| mbGetSource | √ |
| mbUtilSerializeToMHTML | × |
| mbUtilCreateRequestCode | × |
| mbUtilIsRegistered | × |
| mbUtilPrint | × |
| mbUtilBase64Encode | × |
| mbUtilBase64Decode | × |
| mbUtilDecodeURLEscape | × |
| mbUtilEncodeURLEscape | × |
| mbUtilCreateV8Snapshot | × |
| mbUtilPrintToPdf | × |
| mbUtilPrintToBitmap | × |
| mbUtilScreenshot | × |
| mbUtilsSilentPrint | × |
| mbPopupDownloadMgr | × |
| mbPopupDialogAndDownload | √ |
| mbDownloadByPath | × |
| mbGetPdfPageData | × |
| mbCreateMemBuf | × |
| mbFreeMemBuf | × |
| mbPluginListBuilderAddPlugin | × |
| mbPluginListBuilderAddMediaTypeToLastPlugin | × |
| mbPluginListBuilderAddFileExtensionToLastMediaType | × |
| mbEnableHighDPISupport | √ |
| mbRunMessageLoop | √ |
| mbExitMessageLoop | √ |
| mbOnLoadUrlFinish | × |
| mbOnLoadUrlHeadersReceived | × |
| mbOnDocumentReadyInBlinkThread | × |
| mbUtilSetDefaultPrinterSettings | × |
| mbGetContentWidth | × |
| mbGetContentHeight | × |
| mbGetWebViewForCurrentContext | × |
| mbRegisterEmbedderCustomElement | × |
| mbOnNodeCreateProcess | × |
| mbGetGlobalExecByFrame | × |
| mbJsToV8Value | × |
| mbOnThreadIdle | × |
| mbOnBlinkThreadInit | × |
| mbCallBlinkThreadAsync | × |
| mbCallBlinkThreadSync | × |
| mbCallUiThreadSync | × |
| mbCallUiThreadAsync | × |
| mbSetUserKeyValue | √ |
| mbGetUserKeyValue | √ |
| mbGoToOffset | × |
| mbGoToIndex | × |
| mbEditorRedo | × |
| mbEditorUnSelect | × |
| mbGetBlinkMainThreadIsolate | × |
| mbInsertCSSByFrame | × |
| mbWebFrameGetMainWorldScriptContext | × |
| mbOnWillReleaseScriptContext | × |
| mbNetGetReferrer | × |
| mbPostToUiThread | √ |
| mbSetEditable | × |
| mbQueryState | √ |
| mbGetProcAddr | × |