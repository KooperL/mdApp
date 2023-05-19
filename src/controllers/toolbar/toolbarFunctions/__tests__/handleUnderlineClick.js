// const { describe, expect, test } = require("@jest/globals")
// const { handleUnderlineClick } = require("../handleUnderlineClick")
// 
// 
// describe("handleUnderlineClick", () => {
//   let contentEditable
//   let underlineButton 
// 
//   beforeEach(() => {
//     // Set up the test environment
//     contentEditable = document.createElement("div")
//     contentEditable.contentEditable = true
//     document.body.appendChild(contentEditable)
// 
//     boldButton = document.createElement("button")
//     boldButton.addEventListener("click", handleUnderlineClick)
//     document.body.appendChild(boldButton)
//   })
// 
//   afterEach(() => {
//     // Clean up the test environment
//     document.body.removeChild(contentEditable)
//     document.body.removeChild(underlineButton)
//   })
// 
//   test("should underline the selected text within a <p> tag", () => {
//     contentEditable.innerHTML = "<p>This is a test</p>"
//     const pTag = contentEditable.querySelector("p")
//     const textNode = pTag.firstChild
// 
//     // Select the text node
//     const range = document.createRange()
//     range.selectNodeContents(textNode)
//     const selection = window.getSelection()
//     selection.removeAllRanges()
//     selection.addRange(range)
// 
//     // Click the bold button
//     boldButton.click()
// 
//     expect(pTag.innerHTML).toBe("<p><u>This is a test</u></p>")
//   })
// 
//   test("should underline the nested selected text", () => {
// 
//     contentEditable.innerHTML = "<p><strong>This is a test</strong></p>"
//     const pTag = contentEditable.querySelector("p")
//     const textNode = pTag.firstChild
// 
//     // Select the text node
//     const range = document.createRange()
//     range.selectNodeContents(textNode)
//     const selection = window.getSelection()
//     selection.removeAllRanges()
//     selection.addRange(range)
// 
//     // Click the bold button
//     boldButton.click()
// 
//     expect(pTag.innerHTML).toBe("<p><u>This is a test</u></p>")
//   })
// }
