/* @refresh reload */
import { render } from "solid-js/web";

import "./styles.css";
import App from "./App";
import { KeyboardProvider } from "./context/keyboardShortcuts";

render(() => {
  return <>
    <KeyboardProvider>
      <App />
    </KeyboardProvider>
  </>
}, document.getElementById("root") as HTMLElement);
