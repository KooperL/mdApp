// import editableDocument from "../../controllers/editableDocument";
import { Props as jsxProps } from "./types"
import { Props as enhanceProps } from "./../../views/editor/types"

function EditableDocument(props: jsxProps) {
  const { contentEditableRef } = props
  return (
    <div>
      <div
        contentEditable
        ref={contentEditableRef}
        style={`
      border: '1px solid black';
      minHeight: '100px';
      padding: '10px';
`}
      ></div>
    </div>
  )
}

export default function enhance(props: enhanceProps) {
  const { contentEditableRef } = props
  return <EditableDocument contentEditableRef={contentEditableRef} />
}
