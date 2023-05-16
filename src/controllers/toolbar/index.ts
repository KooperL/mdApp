import { Props as exportProps } from "../../components/toolbar/types"
import { Props as importProps } from "./../../views/editor/types"
import { handleBoldClick } from "./toolbarFunctions/handleBoldClick"
import { handleIndentClick } from "./toolbarFunctions/handleIndentClick"
import { handleOutdentClick } from "./toolbarFunctions/handleOutdentClick"
import { handleAddTableClick } from "./toolbarFunctions/handleAddTableClick"
import { handleBlockCodeClick } from "./toolbarFunctions/handleBlockCodeClick"
import { handleUnderlineClick } from "./toolbarFunctions/handleUnderlineClick"
import { handleInlineCodeClick } from "./toolbarFunctions/handleInlineCodeClick"
import { handleInsertLinkClick } from "./toolbarFunctions/handleInsertLinkClick"
import { handleIncreaseFontSizeClick } from "./toolbarFunctions/handleIncreaseFontSizeClick"
import { handleInsertOrderedListClick } from "./toolbarFunctions/handleInsertOrderedListClick"
import { handleInsertUnorderedListClick } from "./toolbarFunctions/handleInsertUnorderedListClick"

function toolbarController(props: importProps): exportProps {
  const { contentEditableRef } = props

  return {
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
  }
}

export default toolbarController
