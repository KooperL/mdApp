export function handleInsertLinkClick() {
  const url = "https://www.google.com"
  const selection = window.getSelection()
  const range = selection.getRangeAt(0)
  const link = document.createElement("a")
  link.href = url
  link.textContent = selection.toString()
  range.deleteContents()
  range.insertNode(link)
}
