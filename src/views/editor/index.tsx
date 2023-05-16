import Toolbar from "../../components/toolbar"
import EditableDocument from "../../components/editableDocument"
import { Props } from "./types"
import editor from "./controller"
import "./style.css"

function Editor(props: Props) {
  const { contentEditableRef } = props

  return (
    <div class="editor">
      <div class="toolbar">
        <Toolbar contentEditableRef={contentEditableRef} />
      </div>
      <div class="document-screen">
        <div class="document-page">
          <EditableDocument contentEditableRef={contentEditableRef} />
        </div>
      </div>
    </div>
  )
}

export default function enhance() {
  const { contentEditableRef } = editor()
  return <Editor contentEditableRef={contentEditableRef} />
}
