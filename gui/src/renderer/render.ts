import {ipcRenderer} from 'electron'

import {HashingError, Payload, Result, Status} from "../interfaces";

const RESULT_PROPERTIES = Object.freeze({
    success: {
        message: 'Success! File matches checksum!',
        className: 'success'
    },
    failure: {
        message: 'Failure! File does not match checksum!',
        className: 'failure'
    },
    error: {
        message: 'Error! Problem hashing the file!',
        className: 'failure'
    }
});

document.addEventListener('DOMContentLoaded', () => {
    calculateListener();
})

const calculateListener = () => {
    const calculateBtn = document.getElementById('calculate_btn') as HTMLButtonElement;
    calculateBtn.addEventListener('click', (event) => {
        event.preventDefault();
        calculateBtn.disabled = true
        const payload = getInputData();

        ipcRenderer.once('hashingReply', (_event, response: Result) => {
            processResponse(response);
            calculateBtn.disabled = false;
        });

        ipcRenderer.send('checkFile', payload)
    })
}

const processResponse = (response: Result) => {
    let message, resultClass;

    switch (response.status) {
        case Status.Success:
            message = RESULT_PROPERTIES.success.message;
            resultClass = RESULT_PROPERTIES.success.className;
            break;
        case Status.Failure:
            message = RESULT_PROPERTIES.failure.message;
            resultClass = RESULT_PROPERTIES.failure.className;
            break;
        default:
            message = RESULT_PROPERTIES.error.message;
            resultClass = RESULT_PROPERTIES.error.className;
    }

    handleErrors(response.errors)
    const errorsPresent = !!(response.errors && response.errors.length);
    handleResultStatus(message, resultClass, errorsPresent)
}

const handleResultStatus: (message: string, className: string, errorsPresent: boolean) => void = (message, className, errorsPresent) => {
    const resultStatus = document.getElementById('result_status') as HTMLParagraphElement;

    resultStatus.className = '';
    resultStatus.textContent = message;

    const resultContainer = document.getElementById('results') as HTMLDivElement;
    if (errorsPresent) {
        resultStatus.textContent = '';
        resultContainer.classList.add('hidden');
    } else {
        resultStatus.classList.add(className);
        resultContainer.classList.remove('hidden')
    }
};

const handleErrors: (errors: Array<HashingError> | undefined) => void = (errors) => {
    const errorsContainer = document.querySelector('.errors-container') as HTMLDivElement;
    const errorsList = document.getElementById('errors_list') as HTMLUListElement;

    errorsList.textContent = '';

    if (errors && errors.length) {
        showErrors(errorsList, errorsContainer, errors)
    } else {
        hideErrors(errorsList, errorsContainer)
    }
}

const showErrors: (
    errorsList: HTMLUListElement,
    errorsContainer: HTMLDivElement,
    errors: Array<HashingError>) => void = (errorsList, errorsContainer, errors) => {
    for (let error of errors) {
        const listItem = createErrorItem(error);
        errorsList.append(listItem);
    }

    errorsContainer.classList.remove('hidden');
}

const hideErrors: (
    errorsList: HTMLUListElement,
    errorsContainer: HTMLDivElement) => void = (errorsList, errorsContainer) => {
    errorsContainer.classList.add('hidden');
};

const createErrorItem: (error: HashingError) => HTMLLIElement = (error) => {
    const errorItem = document.createElement('li');
    errorItem.classList.add('error-item');
    errorItem.textContent = error.message;
    return errorItem;
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

    return new Payload(filePath, checksum)
}