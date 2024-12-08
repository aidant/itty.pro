export class FormErrorMessage extends HTMLElement {
	#control: (EventTarget & Partial<HTMLInputElement>) | null = null;

	#render = () => {
		this.textContent = this.#control?.validationMessage || '';
	};

	#renderAndPreventDefault = (event?: Event) => {
		event?.preventDefault();
		this.#render();
	};

	protected connectedCallback() {
		if (this.parentElement instanceof HTMLLabelElement && this.parentElement.control) {
			this.#control = this.parentElement.control;
		}

		const controlId = this.getAttribute('for');
		const controlFromId = controlId && this.ownerDocument.getElementById(controlId);
		if (controlFromId) {
			this.#control = controlFromId;
		}

		if (!this.#control) return;

		this.#control.addEventListener('input', this.#render);
		this.#control.addEventListener('invalid', this.#renderAndPreventDefault);
		this.#control.addEventListener('x-valid', this.#render);

		if (this.#control.validationMessage === '' && this.textContent) {
			this.#control.setCustomValidity?.(this.textContent);
		} else if (this.#control.validationMessage) {
			this.#render();
		}
	}

	protected disconnectedCallback() {
		if (this.#control) {
			this.#control.removeEventListener('input', this.#render);
			this.#control.removeEventListener('invalid', this.#renderAndPreventDefault);
			this.#control.removeEventListener('x-valid', this.#render);
			this.#control = null;
		}

		this.textContent = '';
	}
}

customElements.define('form-error-message', FormErrorMessage);
