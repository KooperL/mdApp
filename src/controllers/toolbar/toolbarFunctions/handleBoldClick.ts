export function handleBoldClick() {
  const selection = window.getSelection();
  if (selection.rangeCount === 0) return;

  const range = selection.getRangeAt(0);

  let rangeAsString = range.toString() 
  
  let includesOpeningTag = rangeAsString.includes('<strong>')
  let includesClosingTag = rangeAsString.includes('</strong>')

  let fragment = range.extractContents()
  if (includesOpeningTag && !includesClosingTag) {
    // Check if left of strong to start, remove from middle
    
  } else if (!includesOpeningTag && includesClosingTag) {
    // Is opening of range after an unclosed child though
    let openingCount = 0
    let closingCount = 0
    let scope = fragment.parentNode
    while (scope.parentNode !== 'div') {
      if (scope.contains('<strong>')) {
        openingCount++
      }
      if (scope.contains('</strong>')) {
        closingCount++
      }
    }

  } else if (!includesOpeningTag && !includesClosingTag) {
    // Is it a child though?
    
  } else if (includesOpeningTag && includesClosingTag) {
    // remove and add around
    const fragment = range.extractContents();
    fragment.removeChild('strong')

    const strongElement = document.createElement('strong');
    strongElement.appendChild(fragment);
    range.insertNode(strongElement);
  }

  selection.removeAllRanges();
  selection.addRange(range);
}

