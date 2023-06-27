import { invoke } from "@tauri-apps/api/tauri";


function getSelectionCharacterOffsetWithin(element: HTMLElement) {
    var start = 0;
    var end = 0;
    var doc = element.ownerDocument || element.document;
    var win = doc.defaultView || doc.parentWindow;
    var sel;
    if (typeof win.getSelection != "undefined") {
        sel = win.getSelection();
        if (sel.rangeCount > 0) {
            var range = win.getSelection().getRangeAt(0);
            var preCaretRange = range.cloneRange();
            preCaretRange.selectNodeContents(element);
            preCaretRange.setEnd(range.startContainer, range.startOffset);
            start = preCaretRange.toString().length;
            preCaretRange.setEnd(range.endContainer, range.endOffset);
            end = preCaretRange.toString().length;
        }
    } else if ( (sel = doc.selection) && sel.type != "Control") {
        var textRange = sel.createRange();
        var preCaretTextRange = doc.body.createTextRange();
        preCaretTextRange.moveToElementText(element);
        preCaretTextRange.setEndPoint("EndToStart", textRange);
        start = preCaretTextRange.text.length;
        preCaretTextRange.setEndPoint("EndToEnd", textRange);
        end = preCaretTextRange.text.length;
    }
    return { start: start, end: end };
}

async function updateSelection(content_id: string, action: string) {
  const wysiwyg = document.getElementById(content_id)
  const selection: Selection | null = window.getSelection();
  if (!selection) return null

  const {start, end} = getSelectionCharacterOffsetWithin(wysiwyg as HTMLElement);
  // TODO: right to left vs left to right???

  if (!wysiwyg) return null

  let isRelevant = false
  const { children } = wysiwyg
  let iterator = 0
  let shouldIterate = true
  let rollingTicker = 0

  console.log('begin loop')
  while (shouldIterate) {
    const element = children[iterator] ?? null
    let contentLength = element?.textContent?.length ?? 0

    let textBegin = rollingTicker 
    rollingTicker += contentLength
    let textEnd = rollingTicker

    let startCaretIsInThisElement = (
      start >= textBegin &&
      start < textEnd
    )

    let endCaretIsInThisElement = (
      end >= textBegin &&
      end <= textEnd
    )
    console.log(`Loop iterate: ${element}, ${rollingTicker} ${start} ${end} ${startCaretIsInThisElement}, ${endCaretIsInThisElement}, ${isRelevant}`)


    if (startCaretIsInThisElement || isRelevant) {
      isRelevant = true
      console.log(
        'Is updating HTML',
        element.outerHTML,
        contentLength,
        element.textContent,
        element.textContent?.length,
        start,
        end,
        rollingTicker
    )
      const newHTML: string = await invoke("process_styling", { html: element.outerHTML, begin: rollingTicker - start, end: rollingTicker - end, transformation: action })
      
      console.log(newHTML)
      element.outerHTML = newHTML
    }

    if (endCaretIsInThisElement || !element) {
      shouldIterate = false 
      isRelevant = false 
    }

    console.log(`Loop iterate finished: is relevant? ${isRelevant}, is iterating: ${shouldIterate}`)
    iterator ++
  }
  console.log('Loop finished')
}

export {
  updateSelection,
}
