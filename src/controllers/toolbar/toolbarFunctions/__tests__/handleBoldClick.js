const { describe, expect, test } = require("@jest/globals")
const { handleBoldClick } = require("../handleBoldClick.ts")


describe("handleBoldClick", () => {
  let contentEditable
  let boldButton

  beforeEach(() => {
    // Set up the test environment
    contentEditable = document.createElement("div")
    contentEditable.contentEditable = true
    document.body.appendChild(contentEditable)

    boldButton = document.createElement("button")
    boldButton.addEventListener("click", handleBoldClick)
    document.body.appendChild(boldButton)
  })

  afterEach(() => {
    // Clean up the test environment
    document.body.removeChild(contentEditable)
    document.body.removeChild(boldButton)
  })

  test("should bold the entire <p> tag", () => {
    contentEditable.innerHTML = "<p>This is a test</p>"
    const pTag = contentEditable.querySelector("p")
    const textNode = pTag.firstChild

    // Select the text node
    const range = document.createRange()
    range.selectNodeContents(textNode)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe("<strong>This is a test</strong>")
  })

  test("should unbold the entire <p> tag", () => {
    contentEditable.innerHTML = "<p><strong>This is a test</strong></p>"
    const pTag = contentEditable.querySelector("p")
    const boldElement = pTag.querySelector("strong")
    const textNode = boldElement.firstChild

    // Select the text node
    const range = document.createRange()
    range.selectNodeContents(textNode)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe("This is a test")
  })


  test("should bold substring of p tag", () => {
    contentEditable.innerHTML = "<p>This is a test! Testing is soo gooood, I hope you agree author.</p>"
    const pTag = contentEditable.querySelector("p")
    const textNode = pTag.firstChild

    // Select the text node
    const range = document.createRange()
    range.setStart(textNode, 15)
    range.setEnd(textNode, 21)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe("<p>This is a test! <strong>Testing is soo gooood</strong>, I hope you agree author.</p>")
  })


  test("should unbold substring of p tag", () => {
    contentEditable.innerHTML = "<p>This is a test! <strong>Testing is soo gooood</strong>, I hope you agree author.</p>"
    const pTag = contentEditable.querySelector("p")
    const textNode = pTag.firstChild

    // Select the text node
    const range = document.createRange()
    range.setStart(textNode, 15)
    range.setEnd(textNode, 21)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe("<p>This is a test! Testing is soo gooood, I hope you agree author.</p>")
  })

  test("should extend strong tag if range start is not string, but end is", () => {
    contentEditable.innerHTML = "<p>This is a test! <strong>Testing is soo gooood</strong>, I hope you agree author.</p>"
    const pTag = contentEditable.querySelector("p")
    const textNode = pTag.firstChild

    // Select the text node
    const range = document.createRange()
    range.setStart(textNode, 0)
    range.setEnd(textNode, 21)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe("<p><strong>This is a test! Testing is soo gooood</strong>, I hope you agree author.</p>")
  })


  test("should extend strong tag if range start is strong, but end isn't", () => {
    contentEditable.innerHTML = "<p>This is a test! <strong>Testing is soo gooood</strong>, I hope you agree author.</p>"
    const pTag = contentEditable.querySelector("p")
    const textNode = pTag.firstChild

    // Select the text node
    const range = document.createRange()
    range.setStart(textNode, 12)
    range.setEnd(textNode, 40)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe("<p>This is a test! <strong>Testing is soo gooood, I hope you agree author.</strong></p>")
  })


  test("should extend strong tag if range includes entires strong", () => {
    contentEditable.innerHTML = "<p>This is a test! <strong>Testing is soo gooood</strong>, I hope you agree author.</p>"
    const pTag = contentEditable.querySelector("p")
    const textNode = pTag.firstChild

    // Select the text node
    const range = document.createRange()
    range.setStart(textNode, 0)
    range.setEnd(textNode, 40)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe("<p>This is a <strong>test! Testing is soo gooood, I hope you agree author.</strong></p>")
  })

  test("should not extend strong tag over multiple p tags", () => {
    contentEditable.innerHTML = "<p>This is a test! Testing is soo gooood, I hope you agree author.</p><p>I changed my mind, this sucks to write and I need some lorom.</p>"
    const pTag = contentEditable.querySelector("p")
    const textNode = pTag.firstChild

    // Select the text node
    const range = document.createRange()
    range.setStart(textNode, 35)
    range.setEnd(textNode, 60)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe("<p>This is a test! Testing is soo gooood, I hope <strong>you agree author.</strong></p><p><strong>I changed my mind, this sucks</strong> to write and I need some lorom.</p>")
  })


  test("should unbold substring of bold tag", () => {
    contentEditable.innerHTML =
      "<p>This is a <strong>test! Testing is soo gooood, I hope you agree author.</strong></p>"
    const pTag = contentEditable.querySelector("p")

    // Select the text node
    const range = document.createRange()
    range.selectNodeContents(pTag)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe(
      "<p>This is a <strong>test! Testing is soo </strong>gooood, I hope you agree<strong> author.</strong></p>",
    )
  })


  test("should handle nested elements within a <p> tag", () => {
    contentEditable.innerHTML =
      "<p>This is <strong>a <em>test</em></strong></p>"
    const pTag = contentEditable.querySelector("p")
    const strongElement = pTag.querySelector("strong")
    const emElement = strongElement.querySelector("em")
    const textNode = emElement.firstChild

    // Select the text node
    const range = document.createRange()
    range.selectNodeContents(textNode)
    const selection = window.getSelection()
    selection.removeAllRanges()
    selection.addRange(range)

    // Click the bold button
    boldButton.click()

    expect(pTag.innerHTML).toBe(
      "This is <strong>a <u><em>test</em></u></strong>",
    )
  })
})
