#!/usr/bin/env bash

set -xe

if ! command -v wasm-opt &> /dev/null; then
    echo "wasm-opt could not be found"
    exit
fi

if ! command -v wasm-bindgen &> /dev/null; then
    echo "wasm-bindgen could not be found"
    exit
fi

if ! command -v wasm2wat &> /dev/null; then
    echo "wasm2wat could not be found"
    exit
fi

export WASM_PATH="target/wasm32-unknown-unknown/release"

cargo clean --target wasm32-unknown-unknown --release
cargo build --lib --target wasm32-unknown-unknown --release

wasm-bindgen "${WASM_PATH}/codectrl.wasm" --out-dir "${WASM_PATH}/js/" --browser --no-modules --no-typescript
mv "${WASM_PATH}/codectrl.wasm" "${WASM_PATH}/codectrl_bg.wasm"

wasm-opt -O2 --fast-math -o "${WASM_PATH}/js/codectrl_bg.wasm" "${WASM_PATH}/js/codectrl_bg.wasm"
wasm-strip "${WASM_PATH}/js/codectrl_bg.wasm"

cat <<EOF > ${WASM_PATH}/index.html
<!DOCTYPE html>
<html>
<meta http-equiv="Content-Type" content="text/html; charset=utf-8" />

<!-- Disable zooming: -->
<meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no">

<head>
    <title>CodeCTRL</title>
    <style>
        html {
            /* Remove touch delay: */
            touch-action: manipulation;
        }

        body {
            /* Light mode background color for what is not covered by the egui canvas,
            or where the egui canvas is translucent. */
            background: #909090;
        }

        @media (prefers-color-scheme: dark) {
            body {
                /* Dark mode background color for what is not covered by the egui canvas,
                or where the egui canvas is translucent. */
                background: #404040;
            }
        }

        /* Allow canvas to fill entire web page: */
        html,
        body {
            overflow: hidden;
            margin: 0 !important;
            padding: 0 !important;
            height: 100%;
            width: 100%;
        }

        /* Position canvas in center-top: */
        canvas {
            margin-right: auto;
            margin-left: auto;
            display: block;
            position: absolute;
            top: 0%;
            left: 50%;
            transform: translate(-50%, 0%);
        }

        .centered {
            margin-right: auto;
            margin-left: auto;
            display: block;
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            color: #f0f0f0;
            font-size: 24px;
            font-family: Ubuntu-Light, Helvetica, sans-serif;
            text-align: center;
        }

        /* ---------------------------------------------- */
        /* Loading animation from https://loading.io/css/ */
        .lds-dual-ring {
            display: inline-block;
            width: 24px;
            height: 24px;
        }

        .lds-dual-ring:after {
            content: " ";
            display: block;
            width: 24px;
            height: 24px;
            margin: 0px;
            border-radius: 50%;
            border: 3px solid #fff;
            border-color: #fff transparent #fff transparent;
            animation: lds-dual-ring 1.2s linear infinite;
        }

        @keyframes lds-dual-ring {
            0% {
                transform: rotate(0deg);
            }

            100% {
                transform: rotate(360deg);
            }
        }
    </style>
</head>

<body>
    <!-- The WASM code will resize the canvas dynamically -->
    <canvas id="codectrl-root"></canvas>
    <div class="centered" id="center_text">
        <p style="font-size:16px">
            Loading…
        </p>
        <div class="lds-dual-ring"></div>
    </div>

    <script src="./js/codectrl.js"></script>

    <script>
        // We'll defer our execution until the wasm is ready to go.
        // Here we tell bindgen the path to the wasm file so it can start
        // initialization and return to us a promise when it's done.
        console.debug("loading wasm…");
        wasm_bindgen("./codectrl_bg.wasm")
            .then(on_wasm_loaded)
            .catch(on_wasm_error);

        function on_wasm_loaded() {
            console.debug("wasm loaded. starting app…");

            // This call installs a bunch of callbacks and then returns:
            const handle = wasm_bindgen.start("codectrl-root");

            // setTimeout(() => {handle.stop_web(); handle.free())}, 2000)

            console.debug("app started.");
            document.getElementById("center_text").remove();
        }

        function on_wasm_error(error) {
            console.error("Failed to start: " + error);
        }
    </>
</body>

</html>

<!-- Powered by egui: https://github.com/emilk/egui/ -->
EOF