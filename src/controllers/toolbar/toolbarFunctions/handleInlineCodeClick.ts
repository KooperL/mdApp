export function handleInlineCodeClick() {
  const selection = window.getSelection()
  if (selection.rangeCount === 0) return
  const range = selection.getRangeAt(0)
  const codeElement = document.createElement("code")
  codeElement.appendChild(range.extractContents())
  range.insertNode(codeElement)
}
