import { useState } from 'react';
import styles from '../style/FormField.module.css';

const memSizeRegex = /[0-9]+[K|M|G]/

interface Props {
    label: string;
    type: string;
    inputName: string;
    placeholder?: string;
    defaultValue?: any;
    validate?: boolean;
    onValidityChange?: Function;
}

export default function FormField({ label, type, inputName, placeholder, defaultValue, validate, onValidityChange}: Props) {
    const [ valid, setValid ] = useState(true); 

    function isValid(inputText: string) {
        let isValid: boolean = memSizeRegex.test(inputText);
        if(isValid != valid) {
            setValid(isValid);
            if(onValidityChange) onValidityChange();
        }
    }

    return (
        <div className={`${styles.formField}`}>
            <span>{label}</span>
            <input
                className={valid ? '' : styles.invalid}
                placeholder={placeholder}
                aria-label={label}
                type={type}
                name={inputName}
                defaultValue={defaultValue}
                onChange={ type === 'text' && validate
                    ? e => isValid(e.target.value)  
                    : undefined
                }
                checked={ type === 'checkbox'  
                    ? defaultValue 
                    : false
                }

            />
        {type == 'text' && !valid
        ? <p className={styles.inputFormat}>
            Memory size must be an integer number followed by its unit (K, M or G).
          </p> 
        : ''
        }
        </div>
    );
}