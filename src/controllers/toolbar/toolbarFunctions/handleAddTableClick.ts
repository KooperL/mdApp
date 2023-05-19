export function handleAddTableClick () {
  const table = document.createElement("table")
  const tbody = document.createElement("tbody")
  const rows = 2
  const cols = 2

  for (let i = 0; i < rows; i++) {
    const row = document.createElement("tr")
    for (let j = 0; j < cols; j++) {
      const cell = document.createElement("td")
      cell.textContent = "Cell"
      row.appendChild(cell)
    }
    tbody.appendChild(row)
  }

  table.appendChild(tbody)
  document.execCommand("insertHTML", false, table.outerHTML)
}
