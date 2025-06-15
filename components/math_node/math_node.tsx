import React, { CSSProperties, useEffect, useRef, useState } from 'react';
import classNames from 'classnames';
import styles from './math_node.module.scss';
import { MathNode } from '../../bindings/MathNode.ts';
import { MathNodeContent } from '../../bindings/MathNodeContent.ts';
import { TurnTextLineNode } from '../../bindings/TurnTextLineNode.ts';
import { MathJaxProvider, MathJaxNode } from '@yozora/react-mathjax';
import { RefinedMulOrDivOperation } from '../../bindings/RefinedMulOrDivOperation.ts';
import { SpecialMiddleScriptContentTypeNode } from '../../bindings/SpecialMiddleScriptContentTypeNode.ts';
import { UnaryRelationOperatorNode } from '../../bindings/UnaryRelationOperatorNode.ts';
import { RelationOperatorNode } from '../../bindings/RelationOperatorNode.ts';
import { ScriptNode } from '../../bindings/ScriptNode.ts';

const hasMarginList = [
    '÷',
    '/',
    '≌',
    '≡',
    '⊥',
    '≤',
    '≥',
    '<',
    '>',
    '≠',
    '+',
    '*',
    '−',
    '=',
    '×',
    '·',
];

const _relationOperator: {[key: string]: string} = {
    // Binary relations

    // Basic equality and inequality
    IsEqual: "=",
    Equal: "=",
    NotEqual: "≠",
    Greater: ">",
    Less: "<",
    GreaterEqual: "≥",
    LessEqual: "≤",

    // Geometry binary relations
    Collinear: "||",
    Perpendicular: "⊥",
    Equivalent: "≡",
    Similar: "∼",
    Congruent: "≅",

    // Set Theory binary relations
    ElementOf: "∈",
    NotElementOf: "∉",
    SubsetOf: "⊆",
    ProperSubsetOf: "⊂",
    SupersetOf: "⊇",
    ProperSupersetOf: "⊃",
    Disjoint: "⊥",
    Union: "∪",
    Intersection: "∩",
    CartesianProduct: "×",
    SameCardinality: "≈",

    // Number Theory binary relations
    Divides: "|",
    NotDivides: "∤",
    CongruentMod: "≡",
    NotCongruentMod: "≢",
    AreCoprime: "Coprime",

    // Group Theory binary relations
    IsSubgroupOf: "≤",
    IsNormalSubgroupOf: "◁",
    IsIsomorphicTo: "≅",
    IsHomomorphicTo: "→",
    IsQuotientOf: "/",
    IsInCenterOf: "∈Z",
    AreConjugateIn: "~",

    // Ring Theory binary relations
    IsSubringOf: "⊆",
    IsIdealOf: "◁",

    // Topology binary relations
    IsOpenIn: "Open",
    IsClosedIn: "Closed",
    IsHomeomorphicTo: "≃",
    IsDense: "Dense",

    // Category Theory binary relations
    IsMorphismBetween: "→",
    IsIsomorphismIn: "≅",
    IsMonomorphismIn: "↪",
    IsEpimorphismIn: "↠",
    IsNaturalTransformationBetween: "⇒",
    IsAdjunctionBetween: "⊣",
    ComposesTo: "∘",

    // Logic relations
    Implies: "⇒",
    Iff: "⇔",
};

const _unaryRelationOperator: {[key: string]: string} = {
    // Number Theory unary relations
    IsPrime: "Prime",
    IsComposite: "Comp",
    
    // Group Theory unary relations
    HasOrderInGroup: "|·|",
    HasUniqueInverse: "¹",

    // Ring Theory unary relations
    IsPrimeIdeal: "Prime◁",
    IsMaximalIdeal: "Max◁",
    IsPrincipalIdeal: "⟨·⟩",
    IsUnit: "Unit",
    IsIrreducible: "Irr",
    IsPrimeElement: "Prime",
    IsField: "Field",
    IsIntegralDomain: "ID",
    IsUFD: "UFD",
    IsPID: "PID",

    // Topology unary relations
    IsCompact: "Compact",
    IsConnected: "Conn",
    IsContinuous: "Cont",
    Converges: "→",
    IsHausdorff: "Haus",

    // Category Theory unary relations
    IsObjectIn: "∈Ob",
    IsEndomorphismIn: "⟲",
    IsAutomorphismIn: "≅",

    // Set Theory unary operations
    Complement: "ᶜ",
    PowerSet: "P",
};

const _RefinedMulOrDivOperation = {
    Times: '×',
    Dot: '·',
    LittleSpace: '',
    Slash: '/',
    Divide: '÷',
};

const _Bracketed = {
    Round: {
        start: '(',
        end: ')',
    },
    Square: {
        start: '[',
        end: ']',
    },
    Curly: {
        start: '{',
        end: '}',
    },
    Angle: {
        start: '⟨',
        end: '⟩',
    },
    Vertical: {
        start: '|',
        end: '|',
    },
    DoubleVertical: {
        start: '||',
        end: '||',
    },
    Ceiling: {
        start: '⌈',
        end: '⌉',
    },
    Floor: {
        start: '⌊',
        end: '⌋',
    },
    None: {
        start: '',
        end: '',
    },
};

const _AddOrSubOperator = {
    Addition: '+',
    Subtraction: '−',
};

const _SpecialMiddleScriptContent = {
    Hat: '^',
    Dot: '.',
    Tilde: '~',
    Bar: '–',
    Bar2: '¯',
};

//math
export const MathDom = ({
    children,
    lineNo,
    id = '',
    style,
}: {
    children?: React.ReactNode;
    lineNo?: number;
    id?: number | string;
    style: any;
}) => {
    return (
        <div
            className={classNames(styles.math_content)}
            style={style}
            data-id={id}
            line-no={lineNo}
        >
            {children}
        </div>
    );
};

export const Mi = ({
    children,
    id = '',
    _classNames = '',
}: {
    children?: string;
    id?: number | string;
    _classNames?: string;
}) => {
    const MiRef = useRef<HTMLSpanElement>(null);
    const isUp = (content?: string) => {
        return content && typeof content === 'string' && content === content.toUpperCase();
    };

    return (
        <span
            ref={MiRef}
            data-id={id}
            data-content={children}
            data-isup={isUp(children)}
            className={classNames(styles.mi_content, _classNames)}
        >
            <span>{children}</span>
        </span>
    );
};

export const Mspace = ({ width, _classNames = '' }: { width?: string; _classNames?: string }) => {
    const defaultWidth = '1';
    const [_width, setWidth] = useState('0');

    useEffect(() => {
        setWidth((_e) => {
            return width || defaultWidth;
        });
    }, [width]);

    return (
        <span
            style={{
                width: _width + 'em',
            }}
            className={classNames(styles.gap, _classNames)}
        ></span>
    );
};

export const Mn = ({
    children,
    id = '',
    _classNames = '',
}: {
    children?: number | string;
    id?: number | string;
    _classNames?: string;
}) => {
    const MnRef = useRef<HTMLSpanElement>(null);

    return (
        <span
            ref={MnRef}
            data-id={id}
            data-content={children}
            className={classNames(styles.mn_content, _classNames)}
        >
            <span>{children}</span>
        </span>
    );
};

export const Mo = ({
    children,
    id = '',
    isUnit = false,
    _classNames,
}: {
    children?: string;
    id?: number | string;
    isUnit?: boolean;
    _classNames?: string;
}) => {
    const moRef = useRef<HTMLSpanElement>(null);
    const [content, setContent] = useState('');
    const [font, setFont] = useState('s0');

    useEffect(() => {
        if (children?.trim()) {
            setContent(() => {
                return children?.trim().replace(/\-/g, '−').replace(/\'/g, '′') || '';
            });
        } else {
            setContent(children || '');
        }
    }, [children]);

    const hasMargin = () => {
        if (!content || !content.trim()) return false;
        const contentText = moRef.current?.parentElement?.textContent || '';
        return hasMarginList.includes(content.trim()) && !isUnit;
    };

    const fontS2 = () => {
        if (!content || !content.trim()) return false;
        return ['∫', '∫∫', '∫∫∫', '∬', '∭', '∮', '∮∮', '∮∮∮', '∑', '∏'].includes(content.trim());
    };

    const isFencedChangehandle = () => {
        if (!children || !children.trim() || !isFenced() || !moRef.current) return setFont('s0');

        let _size = 0;
        let height = 0;
        let fontsize = parseFloat(window.getComputedStyle(moRef.current).fontSize);
        let centerItem;
        let centerItem_info;

        if (children.trim() === '[' && moRef.current.nextElementSibling) {
            if (moRef.current?.nextElementSibling.textContent === ']') return setFont('s0');
            centerItem = moRef.current.nextElementSibling as HTMLSpanElement;
            centerItem_info = centerItem?.getBoundingClientRect();
            height = centerItem_info.height;
            _size = height / fontsize;
        } else if (children.trim() === ']' && moRef.current.previousElementSibling) {
            if (moRef.current?.previousElementSibling.textContent === '[') return setFont('s0');
            centerItem = moRef.current.previousElementSibling as HTMLSpanElement;
            centerItem_info = centerItem?.getBoundingClientRect();
            height = centerItem_info.height;
            _size = height / fontsize;
        } else {
            return setFont('s0');
        }

        let result = 's0';
        if (_size <= 1.2) {
            result = 's0';
        } else if (_size <= 1.5) {
            result = 's1';
        } else if (_size <= 2) {
            result = 's2';
        } else if (_size <= 3) {
            result = 's3';
        } else if (_size <= 3.2) {
            result = 's4';
        } else {
            result = 'any';
            moRef.current.style.height = height + 'px';
        }
        setFont(result);
        // if (_size <= 3.2) {
        // moRef.current.style.verticalAlign = centerItem_info.height * 0.5 + 'px';
        // }
        if (result === 'any') {
            centerItem.style.verticalAlign = 'baseline';
            if (centerItem.firstElementChild) {
                (centerItem.firstElementChild as HTMLSpanElement).style.verticalAlign = 'baseline';
            }
        } else {
            centerItem.style.verticalAlign = 'middle';
            if (centerItem.firstElementChild) {
                (centerItem.firstElementChild as HTMLSpanElement).style.verticalAlign = '-0.5ex';
            }
        }
    };

    requestAnimationFrame(() => isFencedChangehandle());

    const isFenced = () => {
        if (!children || !children.trim()) return false;
        return ['[', ']'].includes(children.trim());
    };

    return (
        <span
            ref={moRef}
            data-id={id}
            data-font={font}
            data-content={content}
            data-fenced={isFenced()}
            data-hasmargin={hasMargin()}
            data-font2={fontS2()}
            className={classNames(styles.mo_content, _classNames)}
        >
            <span>{isFenced() && font === 'any' ? '' : content}</span>
        </span>
    );
};

export const Mrow = ({
    children,
    id,
    style,
    _classNames = '',
}: {
    children?: React.ReactNode;
    dataType?: 'sub' | 'sup';
    id?: number | string;
    style?: React.CSSProperties;
    _classNames?: string;
}) => {
    const MrowRef = useRef<HTMLSpanElement>(null);

    return (
        <span
            ref={MrowRef}
            data-id={id}
            style={style}
            className={classNames(styles.mrow_content, _classNames)}
        >
            {children}
        </span>
    );
};

export const Msub = ({
    children,
    id,
    _classNames,
}: {
    children?: [React.ReactNode, React.ReactNode] | React.ReactNode;
    id?: number | string;
    _classNames?: string;
}) => {
    const contentRef = useRef<HTMLSpanElement>(null);
    const subItemRef = useRef<HTMLSpanElement>(null);
    const subRef = useRef<HTMLSpanElement>(null);

    useEffect(() => {
        setAlign();
    }, [children]);

    const setAlign = () => {
        if (!subRef.current) return;
        const height = subRef.current.getBoundingClientRect().height;
        subRef.current.style.verticalAlign = -height / 2 + 'px';
    };
    return (
        <span
            className={classNames(styles.msub_content, _classNames)}
            ref={contentRef}
            data-id={id}
        >
            <span className={classNames(styles.msub_content_item)} ref={subItemRef}>
                {children && Array.isArray(children) && children.length > 0
                    ? children[0]
                    : children}
            </span>
            <span className={classNames(styles.msub_content_sub)} ref={subRef}>
                {children && Array.isArray(children) && children.length > 1 ? children[1] : ''}
            </span>
        </span>
    );
};

export const Msup = ({
    children,
    id,
    _classNames,
}: {
    children?: [React.ReactNode, React.ReactNode] | React.ReactNode;
    id?: number | string;
    _classNames?: string;
}) => {
    const contentRef = useRef<HTMLSpanElement>(null);
    const supItemRef = useRef<HTMLSpanElement>(null);
    const supRef = useRef<HTMLSpanElement>(null);
    const isTop = () => {
        if (!children) return;
        let context = contentRef.current?.firstElementChild?.textContent?.trim() || '';
        return ![
            'a',
            'c',
            'e',
            'g',
            'h',
            'i',
            'j',
            'k',
            'l',
            'm',
            'n',
            'o',
            'p',
            'q',
            'r',
            's',
            't',
            'u',
            'v',
            'w',
            'x',
            'y',
            'z',
        ].includes(context);
    };

    useEffect(() => {
        setAlign();
    }, [children]);

    const setAlign = () => {
        if (!supRef.current) return;
        const height = supRef.current.getBoundingClientRect().height;
        const fontSize = parseFloat(window.getComputedStyle(supRef.current).fontSize);
        const hasTable = supRef.current.querySelectorAll(`.${styles.mtable_content}`);
        if (hasTable.length) {
            supRef.current.style.verticalAlign = height / 2 + 'px';
        }
    };

    return (
        <span
            className={classNames(styles.msup_content, _classNames)}
            ref={contentRef}
            data-id={id}
        >
            <span className={classNames(styles.msup_content_item)} ref={supItemRef}>
                {children && Array.isArray(children) && children.length > 0
                    ? children[0]
                    : children}
            </span>
            <span className={classNames(styles.msup_content_sup)} ref={supRef}>
                {children && Array.isArray(children) && children.length > 1 ? children[1] : ''}
            </span>
        </span>
    );
};

interface ChildProps {
    children?: React.ReactNode;
    dataType?: 'sub' | 'sup';
}

export const Msubsup = ({
    children,
    id,
    _classNames,
    scripts_type = 'after',
}: {
    children?: React.ReactNode[] | React.ReactNode;
    id?: number | string;
    _classNames?: string;
    scripts_type?: 'before' | 'after';
}) => {
    const itemContentRef = useRef<HTMLSpanElement>(null);
    const contentRef = useRef<HTMLSpanElement>(null);
    const supRef = useRef<HTMLSpanElement>(null);
    const subRef = useRef<HTMLSpanElement>(null);
    const gapRef = useRef<HTMLSpanElement>(null);
    const [sub, setSub] = useState<React.ReactNode | React.ReactNode[] | null>(null);
    const [sup, setSup] = useState<React.ReactNode | React.ReactNode[] | null>(null);

    useEffect(() => {
        if (!children) return;
        const _sub = filterList('sub');

        const _sup = filterList('sup');
        setSub(() => _sub);
        setSup(() => _sup);
        // setsuper();
        setAlign();
    }, [children]);

    const setAlign = () => {
        if (!contentRef.current || !gapRef.current) return;
        const gap_height = gapRef.current?.getBoundingClientRect().height;
        contentRef.current.style.verticalAlign = gap_height * 2 + 'px';
    };

    const filterList = (type: string) => {
        const list = React.Children.toArray(children).filter((child) => {
            return React.isValidElement(child) && child.props.dataType === type;
        }) as React.ReactNode[];

        const _newList: React.ReactNode[] = [];
        list.forEach((item, index) => {
            _newList.push(item);
            if (index < list.length - 1) {
                _newList.push(<Mo key={'mo' + index}>,</Mo>);
            }
        });

        return _newList;
    };

    const getItemContent = () => {
        let textContent =
            itemContentRef.current?.textContent?.replace(/[\.\¯\~\^]/g, '') ||
            contentRef.current?.previousElementSibling?.textContent?.replace(/[\.\¯\~\^]/g, '') ||
            '';
        return textContent;
    };

    const setsuper = () => {
        // if (scripts_type === 'before' || !contentRef.current || !supRef.current || !gapRef.current) return;
        // const beforeContent = contentRef.current.parentElement;
        // if (!beforeContent) return;
        // const over = beforeContent.querySelector(`.${styles.over}`)
        // const under = beforeContent.querySelector(`.${styles.under}`)
        // const over_height = over?.getBoundingClientRect().height || 0;
        // const under_height = under?.getBoundingClientRect().height || 0;
        // let gap = over_height + under_height;
        // if (over_height) {
        //     gap = gap - 2.3
        // }
        // if (gap <= 0) return;
        // gapRef.current.style.paddingTop = gap + 'px';
    };

    return (
        <span
            className={classNames(styles.msubsup_content, _classNames)}
            ref={contentRef}
            data-id={id}
            data-content={getItemContent()}
        >
            <span className={classNames(styles.msubsup_content_sup)} ref={supRef}>
                {sup}
            </span>
            <span className={classNames(styles.msubsup_content_sub)} ref={subRef}>
                <span>
                    <span className={classNames(styles.msubsup_center_gap)} ref={gapRef}></span>
                    <span className={classNames(styles.sub)}>
                        <span>{sub}</span>
                    </span>
                </span>
            </span>
        </span>
    );
};

export const Mtext = ({
    children,
    id,
    lineNo,
    _classNames,
    style,
}: {
    style?: React.CSSProperties;
    children?: React.ReactNode;
    id?: number | string;
    lineNo?: number | string;
    _classNames?: string;
}) => {
    const MtextRef = useRef<HTMLSpanElement>(null);
    return (
        <span
            ref={MtextRef}
            className={classNames(styles.mtext_content, _classNames)}
            data-id={id}
            line-no={lineNo}
            style={style}
        >
            {children}
        </span>
    );
};

export const Msqrt = ({
    children,
    id,
    _classNames,
}: {
    children?: React.ReactNode | string;
    id?: number | string;
    _classNames?: string;
}) => {
    const contentRef = useRef<HTMLSpanElement>(null);
    const itemRef = useRef<HTMLSpanElement>(null);
    const unitRef = useRef<HTMLSpanElement>(null);
    const extRef = useRef<HTMLSpanElement>(null);

    const [size, setSize] = useState(1);
    const [font, setFont] = useState('s1');
    const listenerMsqrt = () => {
        if (contentRef.current && itemRef.current && itemRef.current.firstElementChild) {
            let fontsize = parseFloat(window.getComputedStyle(contentRef.current).fontSize);
            const _height = itemRef.current.firstElementChild.getBoundingClientRect().height;

            if (_height <= 10) {
                setFont('s0');
            } else if (_height <= 15) {
                setFont('s1');
            } else if (_height <= 20) {
                setFont('s2');
            } else if (_height <= 35) {
                setFont('s3');
            } else if (_height <= 42) {
                setFont('s4');
            } else {
                setFont('any');
                if (extRef.current) {
                    extRef.current.style.height = _height + 0.28 * fontsize + 'px';
                }
            }

            setTimeout(() => {
                if (unitRef.current && itemRef.current) {
                    const sqrt_unit = unitRef.current.getBoundingClientRect();

                    if (_height < sqrt_unit.height && _height > 10) {
                        itemRef.current.style.paddingTop =
                            (sqrt_unit.height - _height) * 0.5 + 'px';
                    }
                }
            }, 0);
        }
    };
    useEffect(() => {
        listenerMsqrt();
    }, [children]);
    return (
        <span
            className={classNames(styles.msqrt_content, _classNames)}
            data-id={id}
            ref={contentRef}
            data-size={size}
        >
            <span>
                <span className={classNames(styles.surd)} ref={unitRef}>
                    <span>
                        {font !== 'any' && (
                            <span className={classNames(styles.sqrt_mo)} data-font={font}>
                                √
                            </span>
                        )}
                        {font === 'any' && (
                            <span className={classNames(styles.stretchy_v)}>
                                <span className={classNames(styles.beg)}>
                                    <span></span>
                                </span>
                                <span className={classNames(styles.ext)} ref={extRef}>
                                    <span></span>
                                </span>
                                <span className={classNames(styles.end)}>
                                    <span></span>
                                </span>
                                <span className={classNames(styles.mark)}></span>
                            </span>
                        )}
                    </span>
                </span>
                <span className={classNames(styles.msqrt_item)} ref={itemRef} data-font={font}>
                    <span
                        style={{
                            display: 'inline-block',
                        }}
                    >
                        {children}
                    </span>
                </span>
            </span>
        </span>
    );
};

export const Mroot = ({
    children,
    id,
    _classNames,
}: {
    children?: [React.ReactNode, React.ReactNode];
    id?: number | string;
    _classNames?: string;
}) => {
    const contentRef = useRef<HTMLSpanElement>(null);
    const nRef = useRef<HTMLSpanElement>(null);
    const [size, setSize] = useState(1);
    const [font, setFont] = useState(false);
    const listenerMsqrt = () => {
        if (contentRef.current) {
            let fontsize = parseFloat(window.getComputedStyle(contentRef.current).fontSize);
            const _size = contentRef.current?.clientHeight / fontsize;

            if (_size === 1) {
                setFont(false);
            } else {
                setFont(true);
            }
            setSize(_size);
        }

        if (nRef.current && contentRef.current && contentRef.current.style) {
            contentRef.current.style.marginLeft = nRef.current.clientWidth + 'px' || '1em';
        }
    };
    useEffect(() => {
        listenerMsqrt();
    }, [children]);
    return (
        <span
            className={classNames(styles.msqrt_content, styles.mroot_content, _classNames)}
            ref={contentRef}
            data-id={id}
            data-font={font}
            data-size={size}
        >
            {children && Array.isArray(children) && children.length > 0 ? children[0] : children}
            <span ref={nRef} className={classNames(styles.n)}>
                {children && Array.isArray(children) && children.length > 1 ? children[1] : ''}
            </span>
        </span>
    );
};

export const Mfrac = ({
    children,
    id,
    _classNames,
}: {
    children?: [React.ReactNode, React.ReactNode] | React.ReactNode;
    id?: number | string;
    _classNames?: string;
}) => {
    return (
        <span className={classNames(styles.mfrac_content, _classNames)} data-id={id}>
            <span className={classNames(styles.mfrac_content_numerator)}>
                {children && Array.isArray(children) && children.length > 0
                    ? children[0]
                    : children}
            </span>
            <span className={classNames(styles.mfrac_content_denominator)}>
                <span>
                    <span className={classNames(styles.line)}></span>
                    <span>
                        <span>
                            {' '}
                            {children && Array.isArray(children) && children.length > 1
                                ? children[1]
                                : ''}
                        </span>
                    </span>
                </span>
            </span>
        </span>
    );
};

export const Mtable = ({
    children,
    id,
    _classNames,
    style,
}: {
    children?: React.ReactNode;
    id?: number | string;
    _classNames?: string;
    style?: React.CSSProperties;
}) => {
    const tableRef = useRef<HTMLSpanElement>(null);
    const [isMatrix, setIsMatrix] = useState<boolean>(false);
    useEffect(() => {
        listener();
    }, [children]);

    const listener = () => {
        if (
            tableRef.current &&
            tableRef.current.parentElement &&
            tableRef.current.parentElement.previousElementSibling
        ) {
            let data = (tableRef.current.parentElement.previousElementSibling as HTMLSpanElement)
                .dataset;
            if (data && data.fenced && data.fenced == 'true') {
                setIsMatrix(true);
            } else {
                setIsMatrix(false);
            }
        }
    };
    return (
        <span
            ref={tableRef}
            data-id={id}
            style={style}
            data-ismatrix={isMatrix}
            className={classNames(styles.mtable_content, _classNames)}
        >
            {children}
        </span>
    );
};

export const Mtablerow = ({ children }: { children?: React.ReactNode }) => {
    return <span className={classNames(styles.mtablerow_content)}>{children}</span>;
};

export const Mtablecell = ({ children }: { children?: React.ReactNode | string }) => {
    return <span className={classNames(styles.mtablecell_content)}>{children}</span>;
};

export const Munder = ({
    children,
    id,
    _classNames,
}: {
    children?: React.ReactNode | React.ReactNode[];
    id?: number | string;
    _classNames?: string;
}) => {
    const contentRef = useRef<HTMLSpanElement>(null);

    const isLimSum = () => {
        if (!children) return false;
        let contentText = contentRef.current?.firstChild?.textContent;
        return ['∑', 'lim'].some((item) => contentText?.includes(item));
    };

    return (
        <span
            className={classNames(styles.munder_content, _classNames)}
            data-id={id}
            data-islimsum={isLimSum()}
            ref={contentRef}
        >
            <span className={classNames(styles.underContent)}>
                <span>{children && Array.isArray(children) ? children[0] : children}</span>
            </span>

            <span className={classNames(styles.under)}>
                <span>{children && Array.isArray(children) && children[1] ? children[1] : ''}</span>
            </span>
        </span>
    );
};

const unicodeValues = [...Array(26).keys()].map((i) => i + 65);

// 使用 String.fromCharCode 生成字母数组
const uppercaseLetters = unicodeValues.map((code) => String.fromCharCode(code));

export const Mover = ({
    children,
    id,
    _classNames,
}: {
    children?: React.ReactNode | React.ReactNode[];
    id?: number | string;
    _classNames?: string;
}) => {
    const contentRef = useRef<HTMLSpanElement>(null);
    const overRef = useRef<HTMLSpanElement>(null);
    const overContentRef = useRef<HTMLSpanElement>(null);
    const [isItalic, setIsItalic] = useState<boolean>(false);
    useEffect(() => {
        getMargin();
        if (contentRef.current) {
            const text = getItemContent(contentRef.current.textContent || '');
            if (text.trim() === 'A') {
                setIsItalic(true);
                return;
            }
        }

        setIsItalic(false);
    }, [children]);

    const getItemContent = (text: string) => {
        let textContent = text.replace(/[\.\~\–\^]/g, '');
        return textContent;
    };

    const getMargin = () => {
        if (!overContentRef.current || !contentRef.current) return;
        const overContent = contentRef.current.getBoundingClientRect();
        overContentRef.current.style.width = overContent.width + 'px';
    };

    const isLimSum = () => {
        if (!children) return false;
        let contentText = contentRef.current?.firstChild?.textContent;
        return ['∑', 'lim'].some((item) => contentText?.includes(item));
    };

    return (
        <span
            className={classNames(styles.mover_content, _classNames)}
            ref={overContentRef}
            data-islimsum={isLimSum()}
            data-id={id}
        >
            <span data-isitalic={isItalic} ref={overRef} className={classNames(styles.over)}>
                {children && Array.isArray(children) && children[1] ? children[1] : ''}
            </span>
            <span ref={contentRef} className={classNames(styles.overContent)}>
                {children && Array.isArray(children) ? children[0] : children}
            </span>
        </span>
    );
};

const componentMap: { [key: string]: (key?: any) => React.ReactElement } = {
    Msub,
    Msqrt,
    Mi,
    Mo,
    Mn,
    MathDom,
    Mrow,
    Msup,
    Msubsup,
    Mtext,
    Mtable,
    Mtablerow,
    Mtablecell,
    Mfrac,
    Mroot,
    Munder,
    Mover,
    Mspace,
    default: Mrow,
};

const Component = ({
    type,
    _props,
    children,
}: {
    type: string;
    _props?: {
        style?: CSSProperties;
        _classNames?: string;
        id?: number | string;
        isUnit?: boolean;
        width?: string;
        dataType?: 'sub' | 'sup';
        lineNo?: string | number;
    };
    children?: React.ReactNode;
}) => {
    const Component = componentMap[type] || componentMap.default;
    return (
        <Component
            {..._props}
            _classNames={
                _props && _props.id && !_props.id.toString().startsWith(':')
                    ? styles.editable_span
                    : ''
            }
        >
            {children}
        </Component>
    );
};

const createStringType = (char: string) => {
    const code = char.charCodeAt(0);
    if ((code >= 65 && code <= 90) || (code >= 97 && code <= 122) || (code >= 945 && code <= 969)) {
        return 'Mi';
    } else if (code >= 48 && code <= 57) {
        return 'Mn';
    } else {
        return 'Mo';
    }
};

const renderRefinedMulOrDivOperation = (type: RefinedMulOrDivOperation) => {
    if (type === 'None') return null;
    const value = Object.values(type)[0];
    if (value === 'LittleSpace') {
        return (
            <Component
                type="Mspace"
                _props={{
                    width: '0.167',
                }}
            ></Component>
        );
    }
    return <Component type="Mo">{_RefinedMulOrDivOperation[value]}</Component>;
};

export const MathSpan = ({ spanData }: { spanData: TurnTextLineNode[] }) => {
    if (!spanData.length) return null;
    return (
        <>
            {spanData.map((item, key) => {
                if (item === 'Empty') return null;
                return renderTurnTextLineNode(item, key + 1);
            })}
        </>
    );
};

const renderTurnTextLineNode = (node: TurnTextLineNode, lineNo: number): React.ReactNode => {
    if (node === 'Empty') return null;
    const key = Object.keys(node)[0];
    switch (key) {
        case 'Math':
            const { Math } = node as Extract<TurnTextLineNode, { Math: [MathNode, string] }>;
            const MathNode = Math[0];
            const Exp = Math[1];
            if (Exp) {
                return (
                    <div className={styles.math_with_explanation} line-no={lineNo} key={lineNo}>
                        {Exp && (
                            <Component
                                type="Mtext"
                                _props={{
                                    style: {
                                        marginLeft: '20px',
                                        marginTop: '10px',
                                        paddingBottom: '0',
                                    },
                                }}
                            >
                                {Exp}
                            </Component>
                        )}
                        <Component
                            type="MathDom"
                            _props={{ style: { marginTop: '0px', marginBottom: '10px' } }}
                        >
                            {renderMathNode(MathNode)}
                        </Component>
                    </div>
                );
            }
            return (
                <Component type="MathDom" _props={{ lineNo: lineNo }} key={lineNo}>
                    {renderMathNode(MathNode)}
                </Component>
            );

        case 'Phrase':
            const { Phrase } = node as Extract<TurnTextLineNode, { Phrase: string }>;
            return (
                <Component type="Mtext" _props={{ lineNo: lineNo }} key={lineNo}>
                    {Phrase}
                </Component>
            );
        case 'Comment':
            return null;
        // const { Comment } = node as Extract<MathNodeContent, { Comment: string }>;
        // return <Component type="Mtext" _props={{ lineNo: lineNo }} key={lineNo}>{Comment}</Component>;
        case 'Latex':
            const { Latex } = node as Extract<TurnTextLineNode, { Latex: string }>;
            return (
                <MathJaxProvider key={lineNo}>
                    <Component type="MathDom" _props={{ lineNo: lineNo }}>
                        <MathJaxNode formula={Latex} inline={true} />
                    </Component>
                </MathJaxProvider>
            );
        case 'PageLink':
            const { PageLink } = node as Extract<TurnTextLineNode, { PageLink: string }>;
            return (
                <Component type="Mtext" _props={{ lineNo: lineNo }} key={lineNo}>
                    {PageLink}
                </Component>
            );
        case 'Image':
            const { Image } = node as Extract<TurnTextLineNode, { Image: string }>;
            return (
                <Component type="Mtext" _props={{ lineNo: lineNo }} key={lineNo}>
                    {Image}
                </Component>
            );
        default:
            return null;
    }
};

const StringMapNode = (_string?: string, id?: string | number) => {
    if (!_string) return null;
    if (_string.length > 1) {
        return (
            <Component
                type="Mrow"
                _props={{
                    id: id,
                    _classNames: styles.editable_span,
                }}
            >
                {[..._string].map((ite, ind) => {
                    return (
                        <Component type={createStringType(ite)} _props={{ isUnit: true }} key={ind}>
                            {ite}
                        </Component>
                    );
                })}
            </Component>
        );
    }

    return (
        <Component
            type={createStringType(_string)}
            _props={{ isUnit: true, id: id, _classNames: styles.editable_span }}
        >
            {_string}
        </Component>
    );
};

const UnderOver = ({
    data,
    type,
}: {
    data: SpecialMiddleScriptContentTypeNode;
    type: 'under' | 'over';
}) => {
    if (typeof data === 'string') {
        const _type = data === 'Bar' ? (type === 'over' ? 'Bar2' : 'Bar') : data;

        return <Component type="Mo">{_SpecialMiddleScriptContent[_type]}</Component>;
    } else if (Object.keys(data)[0] === 'Dot') {
        return Array.from({ length: data.Dot }).map((_, dind) => {
            return (
                <Component type="Mo" key={dind}>
                    {_SpecialMiddleScriptContent['Dot']}
                </Component>
            );
        });
    }
    return null;
};

export const renderMathNode = (node: MathNode): React.ReactNode => {
    if (node.content === 'Empty') return null;
    const key = Object.keys(node.content)[0];

    switch (key) {
        case 'Text':
            const { Text } = node.content as Extract<MathNodeContent, { Text: string }>;

            return (
                <Component
                    type="Mtext"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {Text}
                </Component>
            );
        case 'String':
            const { String } = node.content as Extract<MathNodeContent, { String: string }>;
            return <>{StringMapNode(String, node.id)}</>;
        case 'Integration':
            const { Integration } = node.content as Extract<MathNodeContent, { Integration: { 
                integrand: MathNode; 
                differentials: Array<[MathNode, MathNode | null, MathNode | null]>;
                domain: MathNode | null;
            }}>;
            
            // Extract all differentials from the Integration object
            const differentials = Integration.differentials || [];
                
            // Get domain information if available
            const domain = Integration.domain || null;
            
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {/* Render multiple integral signs based on number of differentials (in reverse order) */}
                    {[...differentials].reverse().map((differential, index) => {
                        const reversedIndex = differentials.length - 1 - index;
                        return (
                            <Component type="Msubsup" key={`integral-${reversedIndex}`}>
                            <Component type="Mo">∫</Component>
                                {/* Show domain under the first integral only (which is the last in reversed order) */}
                                {index === differentials.length - 1 && domain && (
                                <Component
                                    type="Mrow"
                                    _props={{
                                        dataType: 'sub',
                                    }}
                                >
                                        {renderMathNode(domain)}
                                </Component>
                            )}
                                {/* Render upper limit if available */}
                                {differential[2] && (
                                <Component
                                    type="Mrow"
                                    _props={{
                                        dataType: 'sup',
                                    }}
                                >
                                        {renderMathNode(differential[2])}
                                </Component>
                            )}
                                {/* Render lower limit if available */}
                                {differential[1] && (
                                    <Component
                                        type="Mrow"
                                        _props={{
                                            dataType: 'sub',
                                        }}
                                    >
                                        {renderMathNode(differential[1])}
                        </Component>
                                )}
                            </Component>
                        );
                    })}
                        <Component
                            type="Mspace"
                            _props={{
                                width: '0.167',
                            }}
                        ></Component>
                        <Component type="Mrow">{renderMathNode(Integration.integrand)}</Component>
                    
                    {/* Render each differential with its variable (in normal order) */}
                    {differentials.map((differential, index) => (
                        <Component type="Mrow" key={`diff-${index}`}>
                                <Component type="Mspace" _props={{ width: '0.167' }}></Component>
                                <Component type="Mi">d</Component>
                            {renderMathNode(differential[0])}
                            </Component>
                    ))}
                </Component>
            );

        case 'Limit':
            const { Limit } = node.content as Extract<MathNodeContent, { Limit: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    <Component type="Munder">
                        <Component
                            type="Mrow"
                            _props={{
                                style: {
                                    fontStyle: 'normal',
                                },
                            }}
                        >
                            <Component type="Mi">lim</Component>
                        </Component>
                        <Component type="Mrow">
                            <Component type="Mi">{Limit.variable}</Component>
                            <Component type="Mo">→</Component>
                            {renderMathNode(Limit.approaching_value)}
                        </Component>
                    </Component>
                    <Component
                        type="Mspace"
                        _props={{
                            width: '0.167',
                        }}
                    ></Component>
                    {renderMathNode(Limit.function)}
                </Component>
            );
        case 'Differential':
            const { Differential } = node.content as Extract<MathNodeContent, { Differential: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    <Component type="Mfrac">
                        <Component type="Mrow">
                            <Component type="Mi">d</Component>
                            <Component type="Msup">
                                <Component type="Mrow"></Component>
                                <Component type="Mrow">{renderMathNode(Differential.order)}</Component>
                            </Component>
                            {renderMathNode(Differential.target)}
                        </Component>
                        <Component type="Mrow">
                            <Component type="Mi">d</Component>
                            <Component type="Mi">x</Component>
                            <Component type="Msup">
                                <Component type="Mrow"></Component>
                                <Component type="Mrow">{renderMathNode(Differential.order)}</Component>
                            </Component>
                        </Component>
                    </Component>
                </Component>
            );
        case 'QuantifiedExpression':
            const { QuantifiedExpression } = node.content as Extract<MathNodeContent, { QuantifiedExpression: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    <Component type="Mo">
                        {QuantifiedExpression.quantifier === 'Universal' ? '∀' : 
                         QuantifiedExpression.quantifier === 'Existential' ? '∃' : 
                         QuantifiedExpression.quantifier === 'UniqueExistential' ? '∃!' : ''}
                    </Component>
                    <Component
                        type="Mspace"
                        _props={{
                            width: '0.167',
                        }}
                    ></Component>
                    {QuantifiedExpression.variables.map((variable, index) => (
                        <Component type="Mrow" key={index}>
                            {renderMathNode(variable)}
                            {index < QuantifiedExpression.variables.length - 1 && <Component type="Mo">,</Component>}
                        </Component>
                    ))}
                    {QuantifiedExpression.domain && (
                        <Component type="Mrow">
                            <Component
                                type="Mspace"
                                _props={{
                                    width: '0.167',
                                }}
                            ></Component>
                            <Component type="Mo">∈</Component>
                            <Component
                                type="Mspace"
                                _props={{
                                    width: '0.167',
                                }}
                            ></Component>
                            {renderMathNode(QuantifiedExpression.domain)}
                        </Component>
                    )}
                    {QuantifiedExpression.predicate && (
                        <Component type="Mrow">
                            <Component
                                type="Mspace"
                                _props={{
                                    width: '0.167',
                                }}
                            ></Component>
                            <Component type="Mo">:</Component>
                            <Component
                                type="Mspace"
                                _props={{
                                    width: '0.167',
                                }}
                            ></Component>
                            {renderMathNode(QuantifiedExpression.predicate)}
                        </Component>
                    )}
                </Component>
            );
        case 'ScientificNotation':
            const { ScientificNotation } = node.content as Extract<MathNodeContent, { ScientificNotation: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {renderMathNode(ScientificNotation.magnitude)}
                    <Component type="Mo">×</Component>
                    <Component type="Mn">10</Component>
                    <Component type="Msup">
                        <Component type="Mrow"></Component>
                        <Component type="Mrow">{/* Add exponent handling based on style */}</Component>
                    </Component>
                </Component>
            );
        case 'Multiplications':
            const { Multiplications } = node.content as Extract<
                MathNodeContent,
                { Multiplications: any }
            >;

            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {Multiplications.terms.map(
                        ([type, item]: [RefinedMulOrDivOperation, MathNode], ind) => {
                            return (
                                <Component type="Mrow" key={ind}>
                                    {renderRefinedMulOrDivOperation(type)}
                                    {renderMathNode(item)}
                                </Component>
                            );
                        }
                    )}
                </Component>
            );
        case 'Additions':
            const { Additions } = node.content as Extract<MathNodeContent, { Additions: any }>;
            return (
                <>
                    <Component
                        type="Mrow"
                        _props={{
                            id: node.id,
                            _classNames: styles.editable_span,
                        }}
                    >
                        {Additions.terms.map(([type, node], ind) => {
                            return (
                                <Component type="Morw" key={ind}>
                                    {type === 'None' ? null : (
                                        <Component
                                            type="Mo"
                                            _props={{
                                                isUnit: ind == 0,
                                            }}
                                        >
                                            {_AddOrSubOperator[type]}
                                        </Component>
                                    )}
                                    {renderMathNode(node)}
                                </Component>
                            );
                        })}
                    </Component>
                </>
            );
        case 'Division':
            const { Division } = node.content as Extract<MathNodeContent, { Division: any }>;
            let result;
            switch (Division.style) {
                case 'Division':
                    result = (
                        <Component
                            type="Mrow"
                            _props={{
                                id: node.id,
                                _classNames: styles.editable_span,
                            }}
                        >
                            {renderMathNode(Division.numerator)}
                            <Component type="Mo">÷</Component>
                            {renderMathNode(Division.denominator)}
                        </Component>
                    );
                    break;
                case 'Fraction':
                    result = (
                        <Component
                            type="Mfrac"
                            _props={{
                                id: node.id,
                                _classNames: styles.editable_span,
                            }}
                        >
                            <Component type="Mrow">{renderMathNode(Division.numerator)}</Component>
                            <Component type="Mrow">
                                {renderMathNode(Division.denominator)}
                            </Component>
                        </Component>
                    );
                    break;
                case 'Inline':
                    result = (
                        <Component
                            type="Mrow"
                            _props={{
                                id: node.id,
                                _classNames: styles.editable_span,
                            }}
                        >
                            {renderMathNode(Division.numerator)}
                            <Component type="Mo">/</Component>
                            {renderMathNode(Division.denominator)}
                        </Component>
                    );
                    break;
                default:
                    result = '';
            }

            return result;
        case 'SumNotation':
            const { SumNotation } = node.content as Extract<MathNodeContent, { SumNotation: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    <Component type="Munderover">
                        <Component type="Mo">∑</Component>
                        {SumNotation.lower_limit && (
                            <Component type="Mrow">
                                <Component type="Mrow">
                                    {renderMathNode(SumNotation.lower_limit)}
                                </Component>
                            </Component>
                        )}
                        {SumNotation.upper_limit && (
                            <Component type="Mrow">
                                <Component type="Mrow">
                                    {renderMathNode(SumNotation.upper_limit)}
                                </Component>
                            </Component>
                        )}
                    </Component>
                    <Component
                        type="Mspace"
                        _props={{
                            width: '0.167',
                        }}
                    ></Component>
                    <Component type="Mrow">
                        <Component type="Msub">
                            {renderMathNode(SumNotation.summand)}
                            {SumNotation.variable && (
                                <Component type="Mrow">
                                    {renderMathNode(SumNotation.variable)}
                                </Component>
                            )}
                        </Component>
                    </Component>
                </Component>
            );

        case 'ProductNotation':
            const { ProductNotation } = node.content as Extract<
                MathNodeContent,
                { ProductNotation: any }
            >;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    <Component type="Munderover">
                        <Component type="Mo">∏</Component>
                        {ProductNotation.lower_limit && (
                            <Component type="Mrow">
                                <Component type="Mrow">
                                    {renderMathNode(ProductNotation.lower_limit)}
                                </Component>
                            </Component>
                        )}
                        {ProductNotation.upper_limit && (
                            <Component type="Mrow">
                                <Component type="Mrow">
                                    {renderMathNode(ProductNotation.upper_limit)}
                                </Component>
                            </Component>
                        )}
                    </Component>
                    <Component
                        type="Mspace"
                        _props={{
                            width: '0.167',
                        }}
                    ></Component>
                    <Component type="Mrow">
                        <Component type="Msub">
                            {renderMathNode(ProductNotation.multiplicand)}
                            {ProductNotation.variable && (
                                <Component type="Mrow">
                                    {renderMathNode(ProductNotation.variable)}
                                </Component>
                            )}
                        </Component>
                    </Component>
                </Component>
            );

        case 'Fraction':
            const { Fraction } = node.content as Extract<MathNodeContent, { Fraction: any }>;
            return (
                <Component
                    type="Mfrac"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    <Component type="Mrow">{renderMathNode(Fraction.numerator)}</Component>
                    <Component type="Mrow">{renderMathNode(Fraction.denominator)}</Component>
                </Component>
            );
        case 'Bracketed':
            const { Bracketed } = node.content as Extract<MathNodeContent, { Bracketed: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    <Component type="Mo">{_Bracketed[Bracketed.style].start}</Component>
                    {renderMathNode(Bracketed.inner)}
                    <Component type="Mo">{_Bracketed[Bracketed.style].end}</Component>
                </Component>
            );
        case 'Matrix':
            const { Matrix } = node.content as Extract<MathNodeContent, { Matrix: any }>;
            const maxLength = Math.max(...Matrix.rows.map((row) => row.length));
            type MathNodeOrNull = MathNode | null;
            const filledRows: MathNodeOrNull[][] = [...Matrix.rows].map((row: MathNodeOrNull[]) => {
                while (row.length < maxLength) {
                    row.push(null);
                }
                return row;
            });

            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                        style: {
                            verticalAlign: filledRows.length > 1 ? 'middle' : 'baseline',
                        },
                    }}
                >
                    <Component type="Mo">[</Component>
                    <Component
                        type="Mrow"
                        _props={{
                            style: {
                                boxSizing: 'border-box',
                                display: 'inline-block',
                            },
                        }}
                    >
                        <Component type="Mtable">
                            {filledRows.map((item, rowIndex) => {
                                return (
                                    <Component type="Mtablerow" key={rowIndex}>
                                        {item.map((cell, cellIndex) => {
                                            return (
                                                <Component type="Mtablecell" key={cellIndex}>
                                                    {cell && renderMathNode(cell)}
                                                    <span
                                                        className={classNames(styles.tstrut)}
                                                    ></span>
                                                </Component>
                                            );
                                        })}
                                    </Component>
                                );
                            })}
                        </Component>
                    </Component>
                    <Component type="Mo">]</Component>
                </Component>
            );

        case 'UnaryPostfixOperation':
            const { UnaryPostfixOperation } = node.content as Extract<
                MathNodeContent,
                { UnaryPostfixOperation: any }
            >;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {renderMathNode(UnaryPostfixOperation.parameter)}
                    {renderMathNode(UnaryPostfixOperation.operator)}
                </Component>
            );
        case 'UnaryPrefixOperation':
            const { UnaryPrefixOperation } = node.content as Extract<MathNodeContent, { UnaryPrefixOperation: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {renderMathNode(UnaryPrefixOperation.operator)}
                    {renderMathNode(UnaryPrefixOperation.parameter)}
                </Component>
            );
        case 'Abs':
            const { Abs } = node.content as Extract<MathNodeContent, { Abs: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    <Component type="Mo">|</Component>
                    {renderMathNode(Abs.parameter)}
                    <Component type="Mo">|</Component>
                </Component>
            );
        case 'Power':
            const { Power } = node.content as Extract<MathNodeContent, { Power: any }>;
            return (
                <Component
                    type="Msup"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {renderMathNode(Power.base)}
                    <Component type="Mrow">{renderMathNode(Power.exponent)}</Component>
                </Component>
            );
        case 'FunctionCall':
            const { FunctionCall } = node.content as Extract<
                MathNodeContent,
                { FunctionCall: any }
            >;

            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                        style: {
                            alignItems: 'center',
                        },
                    }}
                >
                    {renderMathNode(FunctionCall.name)}
                    <Component type="Mo">(</Component>
                    {FunctionCall.parameters.map((item, ind) => {
                        return (
                            <Component type="Mrow" key={ind}>
                                {renderMathNode(item)}
                                {ind < FunctionCall.parameters.length - 1 && (
                                    <Component type="Mo">,</Component>
                                )}
                            </Component>
                        );
                    })}
                    <Component type="Mo">)</Component>
                </Component>
            );
        case 'Quantity':
            const { Quantity } = node.content as Extract<MathNodeContent, { Quantity: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {StringMapNode(Quantity.number, node.id)}
                    {Quantity.scientific_notation && renderMathNode(Quantity.scientific_notation)}
                    {Quantity.unit && renderMathNode(Quantity.unit)}
                </Component>
            );
        case 'Identifier':
            const { Identifier } = node.content as Extract<MathNodeContent, { Identifier: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {Identifier.pre_script && renderScriptNode(Identifier.pre_script, node.id, 0)}
                    {Identifier.mid_script ? (
                        <Component type="Mover">
                            <Component type="Munder">
                                {StringMapNode(Identifier.body)}
                                {Identifier.mid_script.sub_script && (
                                    <Component type="Mrow">
                                        {Identifier.mid_script.sub_script
                                            .slice()
                                            .reverse()
                                            .map((ite, ind) => {
                                                return (
                                                    <Component type="Mrow" key={ind}>
                                                        <UnderOver
                                                            data={ite}
                                                            type="under"
                                                        ></UnderOver>
                                                    </Component>
                                                );
                                            })}
                                    </Component>
                                )}
                            </Component>
                            <Component type="Mrow">
                                {Identifier.mid_script.super_script
                                    .slice()
                                    .reverse()
                                    .map((ite, ind) => {
                                        return (
                                            <Component type="Mrow" key={ind}>
                                                <UnderOver data={ite} type="over"></UnderOver>
                                            </Component>
                                        );
                                    })}
                            </Component>
                        </Component>
                    ) : (
                        StringMapNode(Identifier.body)
                    )}
                    {(Identifier.post_script) &&
                        renderScriptNode(
                            Identifier.post_script,
                            node.id,
                            Identifier.primes
                        )}
                </Component>
            );

        case 'Unit':
            const { Unit } = node.content as Extract<MathNodeContent, { Unit: any }>;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {/* {renderMathNode(Unit.original_form)} */}
                    {/* <Component type="Mi">/</Component> */}
                    {renderMathNode(Unit.flattened_form)}
                </Component>
            );
        case 'Relationship':
            const { Relationship } = node.content as Extract<
                MathNodeContent,
                { Relationship: any }
            >;
            
            // Get the operator symbol based on the operator type
            const getRelationOperatorSymbol = (operator: RelationOperatorNode): string => {
                if (typeof operator === 'string') {
                    return _relationOperator[operator] || '';
                } else if (operator && typeof operator === 'object' && 'Custom' in operator) {
                    return operator.Custom || '';
                }
                return '';
            };
            
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {renderMathNode(Relationship.lhs)}
                    <Component type="Mo">
                        {getRelationOperatorSymbol(Relationship.operator)}
                    </Component>
                    {renderMathNode(Relationship.rhs)}
                </Component>
            );
        case 'UnaryRelationship':
            const { UnaryRelationship } = node.content as Extract<
                MathNodeContent,
                { UnaryRelationship: { subject: MathNode; predicate: UnaryRelationOperatorNode } }
            >;

             // Get the operator symbol based on the operator type
             const getUnaryRelationOperatorSymbol = (operator: UnaryRelationOperatorNode): string => {
                if (typeof operator === 'string') {
                    return _relationOperator[operator] || '';
                } else if (operator && typeof operator === 'object' && 'Custom' in operator) {
                    return operator.Custom || '';
                }
                return '';
            };
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    <Component type="Mrow">
                        {UnaryRelationship && UnaryRelationship.predicate && _unaryRelationOperator[getUnaryRelationOperatorSymbol(UnaryRelationship.predicate)] && _unaryRelationOperator[getUnaryRelationOperatorSymbol(UnaryRelationship.predicate)].length > 2 
                            ? <Component type="Mtext">{_unaryRelationOperator[getUnaryRelationOperatorSymbol(UnaryRelationship.predicate)]}</Component>
                            : UnaryRelationship && UnaryRelationship.predicate && _unaryRelationOperator[getUnaryRelationOperatorSymbol(UnaryRelationship.predicate)] 
                              ? <Component type="Mo">{_unaryRelationOperator[getUnaryRelationOperatorSymbol(UnaryRelationship.predicate)]}</Component>
                              : null
                        }
                    </Component>
                    <Component type="Mo">(</Component>
                    {UnaryRelationship && UnaryRelationship.subject ? renderMathNode(UnaryRelationship.subject) : null}
                    <Component type="Mo">)</Component>
                </Component>
            );
        case 'VariableDefinition':
            const { VariableDefinition } = node.content as Extract<
                MathNodeContent,
                { VariableDefinition: any }
            >;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {renderMathNode(VariableDefinition.name)}
                    {VariableDefinition.definition && <Component type="Mo">=</Component>}
                    {VariableDefinition.definition && renderMathNode(VariableDefinition.definition)}
                </Component>
            );

        case 'FunctionDefinition':
            const { FunctionDefinition } = node.content as Extract<
                MathNodeContent,
                { FunctionDefinition: any }
            >;
            return (
                <Component
                    type="Mrow"
                    _props={{
                        id: node.id,
                        _classNames: styles.editable_span,
                    }}
                >
                    {renderMathNode(FunctionDefinition.custom_function)}
                    {FunctionDefinition.definition && <Component type="Mo">=</Component>}
                    {FunctionDefinition.definition && renderMathNode(FunctionDefinition.definition)}
                </Component>
            );
    }
    return null;
};

function renderScriptNode(Script: ScriptNode, id: string, primes: number) {
    return (
        <Component
            type="Msubsup"
            _props={{
                id,
                _classNames: styles.editable_span,
            }}
        >
            {primes > 0 && !Script.superscripts.length ? (
                <Mrow dataType="sup" key="dot">
                    {[...Array(primes)].map((_, ind) => {
                        return (
                            <Component type="Mo" key={'dot' + ind}>
                                '
                            </Component>
                        );
                    })}
                </Mrow>
            ) : null}
            {Script.subscripts.map((item, ind) => {
                return (
                    <Mrow dataType="sub" key={'_sub' + ind}>
                        {renderMathNode(item)}
                    </Mrow>
                );
            })}
            {Script.superscripts.map((item, ind) => {
                return (
                    <Mrow dataType="sup" key={'_sup' + ind}>
                        {ind === 0 &&
                            [...Array(primes)].map((_, ind) => {
                                return (
                                    <Component type="Mo" key={'dot' + ind}>
                                        '
                                    </Component>
                                );
                            })}
                        {renderMathNode(item)}
                    </Mrow>
                );
            })}
        </Component>
    );
}