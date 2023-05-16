export function handleBlockCodeClick() {
  const selection = window.getSelection()
  if (selection.rangeCount === 0) return
  const range = selection.getRangeAt(0)
  const preElement = document.createElement("pre")
  preElement.appendChild(range.extractContents())
  range.insertNode(preElement)
}
