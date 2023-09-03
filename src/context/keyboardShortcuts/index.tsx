import { createContext, createSignal, onCleanup } from 'solid-js';

interface KeyboardContextValue {
  undo: (id: string) => void;
  redo: (id: string) => void;
  registerTextArea: (id: string, initialValue: string) => void;
  applyChange: (id: string, newValue: string) => void;
}

export const KeyboardContext = createContext<KeyboardContextValue>(
  undefined
);

export const KeyboardProvider = (props: any) => {
  const { children } = props
  const textAreas: Record<string, string[]> = {}; // Store undo stacks for each text area

  const history: Record<string, (string | null)[]> = {}; // Store history of changes for each text area

  const currentIndex: Record<string, number> = {}; // Store the current index in the history stack

  const registerTextArea = (id: string, initialValue: string) => {
    if (!textAreas[id]) {
      textAreas[id] = [initialValue];
      history[id] = [initialValue];
      currentIndex[id] = 0;
    }
  };

  const applyChange = (id: string, newValue: string) => {
    if (textAreas[id]) {
      const currentIndexForId = currentIndex[id];
      const historyForId = history[id];

      // If we're not at the latest change, clear the redo stack
      if (currentIndexForId < historyForId.length - 1) {
        historyForId.splice(currentIndexForId + 1);
      }

      historyForId.push(newValue);
      currentIndex[id] = currentIndexForId + 1;
      textAreas[id].push(newValue);
    }
  };

  const undo = (id: string) => {
    if (textAreas[id]) {
      const currentIndexForId = currentIndex[id];
      if (currentIndexForId > 0) {
        currentIndex[id] = currentIndexForId - 1;
        const previousValue = history[id][currentIndexForId - 1];
        textAreas[id][textAreas[id].length - 1] = previousValue || '';
      }
    }
  };

  const redo = (id: string) => {
    if (textAreas[id]) {
      const currentIndexForId = currentIndex[id];
      if (currentIndexForId < history[id].length - 1) {
        currentIndex[id] = currentIndexForId + 1;
        const nextValue = history[id][currentIndexForId + 1];
        textAreas[id][textAreas[id].length - 1] = nextValue || '';
      }
    }
  };


  return (
    <KeyboardContext.Provider value={{ undo, redo, registerTextArea, applyChange }}>
      {children}
    </KeyboardContext.Provider>
  );
};
