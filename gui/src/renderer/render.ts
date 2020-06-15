import { ipcRenderer } from 'electron'

document.addEventListener('DOMContentLoaded', () => {
    calculateListener();
})

const calculateListener = () => {
    const calculateBtn = document.getElementById('calculate_btn');
    if (calculateBtn) {
        calculateBtn.addEventListener('click', (event) => {
            event.preventDefault();
            const {checksum, filePath} = getInputData();

            ipcRenderer.once('hashingReply', (_event, response: object) => {
                processResponse(response);
            });

            ipcRenderer.send('checkFile', {checksum: checksum, filePath: filePath})
        })
    }

}

const processResponse = (_response: object) => {
    // placeholder
    console.log('Processing response...');
}

const getInputData = () => {
    let filePath, checksum;
    const filePathEl = document.getElementById('file_path') as HTMLInputElement;
    if (filePathEl) {
        filePath = filePathEl.value;
    }

    const checksumEl = document.getElementById('checksum_text') as HTMLInputElement;
    if (checksumEl) {
        checksum = checksumEl.value;
    }

    return {
        filePath: filePath,
        checksum: checksum
    }
}
