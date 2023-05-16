export function handleAddTableClick () {
  const table = document.createElement("table")
  const tbody = document.createElement("tbody")

  for (let i = 0; i < 2; i++) {
    const row = document.createElement("tr")
    for (let j = 0; j < 2; j++) {
      const cell = document.createElement("td")
      cell.textContent = "Cell"
      row.appendChild(cell)
    }
    tbody.appendChild(row)
  }

  table.appendChild(tbody)
  document.execCommand("insertHTML", false, table.outerHTML)
}
