import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { updateSelection } from "./utils/getRangeSelection";
import { getAllChildrenInSelection } from "./utils/getAllChildrenInSelection";
import { keyPresshandler } from "./utils/keyboardEventHandler";


function App() {
  const [font, setFont] = createSignal("");
  const contentID = "wysiwyg"

  /**
    * TODO for MVP
    * Checkbox
    * Table
    * Heading
    * Strikethrough
    * Underline
  **/

  return (
    <div class="container">
      <form
        class="row"
        onSubmit={async (e) => {
          e.preventDefault();
          if (!document.querySelector(`#${contentID}`)) throw new Error()
        }}
      >
        <div class="toolbar">
          <button onclick={() => {
            updateSelection(contentID, "bold")
          }}
        >
          <b>B</b>
        </button>
        <button onclick={() => {
            updateSelection(contentID, "emphasise")
          }}
        >
          <i>I</i>
        </button>
        <button onclick={() => {
            updateSelection(contentID, "ordered-list")
          }}
        >
          <span>1. __</span>
        </button>
        <button onclick={() => {
            updateSelection(contentID, "unordered-list")
          }}
        >
          <span>⋅ __</span>
        </button>
        <button onclick={() => {
            updateSelection(contentID, "code")
          }}
        >
          <code>\{"</>"}</code>
        </button>
        <button onclick={() => {
            updateSelection(contentID, "superscript")
          }}
        >
          <span>a<sup>x</sup></span>
        </button>
        <button onclick={() => {
            updateSelection(contentID, "subscript")
          }}
        >
          <span>a<sub>x</sub></span>
        </button>
        <select id="selecth1FontFamily" name="selectFontFamily" onchange={(e) => {
          const elem = document.getElementById(contentID)
          if (!elem) return
          elem.style.fontFamily = e.target.value
          }}>
          <option>Serif</option>
          <option>Arial</option>
          <option>Sans-Serif</option>                                  
          <option>Tahoma</option>
          <option>Verdana</option>
          <option>Lucida Sans Unicode</option>                               
        </select>
        </div>
        <div id="content-container">
          <div
            id={contentID}
            contenteditable={true}
          >
            <p 
              contenteditable={true}
              onKeyDown={keyPresshandler}
            >&#8203;</p>
          </div>
        </div>
      </form>
    </div>
  );
}

// Spaces: \u{3164} \u{200b}

export default App;
