import { createSignal } from "solid-js";
import "./App.css";
import { updateSelection } from "./utils/getRangeSelection";
import { keyPresshandler } from "./utils/keyboardEventHandler";
import TextArea from "./components/TextArea";


function App() {
  document.title = 'mdApp'
  const [font, setFont] = createSignal("");
  const contentID = "wysiwyg"

/**
  * TODO
  * Table (table tr th/td)
  * Button to selected if caret is over element
  * Raw paste
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
            <div class="button-container">
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
              <i style={{'font-family': 'serif'}}>I</i>
            </button>
            <button onclick={() => {
                updateSelection(contentID, "strikethrough")
              }}
            >
              <s>S</s>
            </button>
            <button onclick={() => {
                updateSelection(contentID, "underline")
              }}
            >
              <u>U</u>
            </button>
            <button onclick={() => {
                updateSelection(contentID, "ordered-list")
              }}
            >
              <span>1.</span>
            </button>
            <button onclick={() => {
                updateSelection(contentID, "unordered-list")
              }}
            >
              <span>●</span>
            </button>
            <button onclick={() => {
                updateSelection(contentID, "code")
              }}
            >
              <code style={{'font-size': 'smaller'}}>{"</>"}</code>
            </button>
            <button onclick={() => {
                updateSelection(contentID, "check")
              }}
            >
              <span>✓</span>
            </button>
            <button onclick={() => {
                updateSelection(contentID, "superscript")
              }}
            >
              <span>a<sup>x</sup></span>
            </button>
            <button onclick={() => {
                updateSelection(contentID, "heading")
              }}
            >
              <span># _</span>
            </button>
            <button onclick={() => {
                updateSelection(contentID, "subscript")
              }}
            >
              <span>a<sub>x</sub></span>
            </button>
            </div>
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
              <TextArea
                id={contentID}
              >
                <p 
                  contenteditable={true}
                  onKeyDown={keyPresshandler}
                >&#8203;</p>
              </TextArea>
          </div>
        </form>
      </div>
  );
}

export default App;
