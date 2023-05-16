export function handleIncreaseFontSizeClick() {
  const selection = window.getSelection()
  if (selection.rangeCount === 0) return
  const range = selection.getRangeAt(0)
  const parentElement = range.commonAncestorContainer.parentElement // Check if the common ancestor is a code, pre, or table element

  // Check if the common ancestor is a code, pre, or table element
  if (
    parentElement.closest("code") ||
    parentElement.closest("pre") ||
    parentElement.closest("table")
  ) {
    return
  }

  document.execCommand("fontSize", false, "2")
}
