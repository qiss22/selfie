kotlin {{
            js(IR) {{
                browser {{
                    webpackTask {{
                        outputFileName = "selfie-kmp.js"
                    }}
                }}
                binaries.executable()
            }}
        }}