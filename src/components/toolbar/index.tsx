import { Props as jsxProps } from "./types"
import { Props as enhanceProps } from "./../../views/editor/types"
import ToolbarController from "../../controllers/toolbar"
import "./style.css"

function Toolbar(props: jsxProps) {
  const {
    handleBoldClick,
    handleIndentClick,
    handleOutdentClick,
    handleAddTableClick,
    handleBlockCodeClick,
    handleUnderlineClick,
    handleInlineCodeClick,
    handleInsertLinkClick,
    handleIncreaseFontSizeClick,
    handleInsertOrderedListClick,
    handleInsertUnorderedListClick,
  } = props

  return (
    <div>
      <button onClick={handleBoldClick}>Bold</button>
      <button onClick={handleUnderlineClick}>Underline</button>
      <button onClick={handleIncreaseFontSizeClick}>Increase Font Size</button>
      <button onClick={handleInsertOrderedListClick}>Ordered List</button>
      <button onClick={handleInsertUnorderedListClick}>Unordered List</button>
      <button onClick={handleIndentClick}>Indent</button>
      <button onClick={handleOutdentClick}>Outdent</button>
      <button onClick={handleInsertLinkClick}>Insert Link</button>
      <button onClick={handleInlineCodeClick}>Inline code</button>
      <button onClick={handleBlockCodeClick}>Code block</button>
      <button onClick={handleAddTableClick}>Blank table</button>
    </div>
  )
}

export default function enhance(props: enhanceProps) {
  const {
    handleBoldClick,
    handleIndentClick,
    handleOutdentClick,
    handleAddTableClick,
    handleBlockCodeClick,
    handleUnderlineClick,
    handleInlineCodeClick,
    handleInsertLinkClick,
    handleIncreaseFontSizeClick,
    handleInsertOrderedListClick,
    handleInsertUnorderedListClick,
    ContentEditableRef,
  } = ToolbarController(props)
  return (
    <Toolbar
      ContentEditableRef={ContentEditableRef}
      handleBoldClick={handleBoldClick}
      handleIndentClick={handleIndentClick}
      handleOutdentClick={handleOutdentClick}
      handleAddTableClick={handleAddTableClick}
      handleBlockCodeClick={handleBlockCodeClick}
      handleUnderlineClick={handleUnderlineClick}
      handleInlineCodeClick={handleInlineCodeClick}
      handleInsertLinkClick={handleInsertLinkClick}
      handleIncreaseFontSizeClick={handleIncreaseFontSizeClick}
      handleInsertOrderedListClick={handleInsertOrderedListClick}
      handleInsertUnorderedListClick={handleInsertUnorderedListClick}
    />
  )
}
