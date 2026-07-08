use crate::circuit::Circuit;
use crate::circuit_view::CircuitView;
use std::collections::BTreeSet;
use std::fmt::Debug;

pub struct Testbench<'a> {
    circuit: &'a dyn CircuitView,
    cycles: Option<usize>,
    period_ps: Option<f64>,
    stimuli: Vec<StimulusSpec>,
    observe: Vec<String>,
}

pub(crate) struct NormalizedTestbench<'a> {
    pub(crate) name: &'a str,
    pub(crate) circuit: &'a dyn CircuitView,
    pub(crate) cycles: usize,
    pub(crate) period_ps: f64,
    pub(crate) signals: Vec<SignalPattern>,
    pub(crate) observe: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct SignalPattern {
    pub(crate) name: String,
    pub(crate) values: Vec<u8>,
    pub(crate) phase: f64,
}

enum StimulusSpec {
    Signal {
        name: String,
        values: Vec<u64>,
        phase: f64,
    },
    Signals {
        names: Vec<String>,
        values: Vec<u64>,
        phase: f64,
    },
    Constant {
        name: String,
        value: u64,
        phase: f64,
    },
    Pulse {
        name: String,
        cycles: Vec<usize>,
        phase: f64,
    },
    Toggle {
        name: String,
        cycles: Vec<usize>,
        phase: f64,
    },
}

pub trait TestbenchBackend {
    /// Testbench を各シミュレータ向けの文字列へ変換する.
    fn generate(&self, testbench: &Testbench<'_>) -> String;
}

impl<'a> Testbench<'a> {
    /// 対象回路から Testbench builder を作る.
    ///
    /// VCD/CSV などの出力ファイル名には回路名を使う.
    pub fn new<const N_I: usize, const N_CI: usize, const N_O: usize, const N_CO: usize>(
        circuit: &'a Circuit<N_I, N_CI, N_O, N_CO>,
    ) -> Self {
        Self {
            circuit,
            cycles: None,
            period_ps: None,
            stimuli: Vec::new(),
            observe: Vec::new(),
        }
    }

    /// シミュレーション全体のサイクル数を指定する.
    ///
    /// 入力パターン後の静穏期間もこのサイクル数に含める.
    pub fn cycles(mut self, cycles: usize) -> Self {
        assert!(cycles > 0, "testbench cycles must be greater than zero");
        self.cycles = Some(cycles);
        self
    }

    /// 1 サイクルの長さをps単位で指定する.
    ///
    /// 物理バックエンドでは入力パルスの時刻計算に使う.
    pub fn period_ps(mut self, period_ps: f64) -> Self {
        assert!(
            period_ps.is_finite() && period_ps > 0.0,
            "testbench period_ps must be a positive finite number"
        );
        self.period_ps = Some(period_ps);
        self
    }

    /// 1-bit 入力信号を 0/1 の整数列で指定する.
    ///
    /// 指定列が `cycles` より短い場合は末尾を 0 で埋める.
    pub fn signal<I, V>(mut self, name: &str, values: I, phase: f64) -> Self
    where
        I: IntoIterator<Item = V>,
        V: TryInto<u64>,
        V::Error: Debug,
    {
        self.stimuli.push(StimulusSpec::Signal {
            name: name.to_string(),
            values: collect_numbers(values),
            phase,
        });
        self
    }

    /// 複数の 1-bit 入力信号を整数列からまとめて指定する.
    ///
    /// `names` は MSB first として扱う. 例えば `["cin", "b", "a"]` に
    /// 値 `0b101` を与えると `cin=1, b=0, a=1` になる.
    pub fn signals<N, I, V>(mut self, names: N, values: I, phase: f64) -> Self
    where
        N: IntoIterator,
        N::Item: AsRef<str>,
        I: IntoIterator<Item = V>,
        V: TryInto<u64>,
        V::Error: Debug,
    {
        self.stimuli.push(StimulusSpec::Signals {
            names: names
                .into_iter()
                .map(|name| name.as_ref().to_string())
                .collect(),
            values: collect_numbers(values),
            phase,
        });
        self
    }

    /// 全サイクルで同じ値を持つ入力信号を指定する.
    ///
    /// クロックを毎サイクル入れたい場合のショートカットとしても使う.
    pub fn constant<V>(mut self, name: &str, value: V, phase: f64) -> Self
    where
        V: TryInto<u64>,
        V::Error: Debug,
    {
        self.stimuli.push(StimulusSpec::Constant {
            name: name.to_string(),
            value: value
                .try_into()
                .expect("constant value must be convertible to u64"),
            phase,
        });
        self
    }

    /// 指定したサイクルだけ 1 になる入力信号を指定する.
    ///
    /// それ以外のサイクルは 0 になる.
    pub fn pulse<I>(mut self, name: &str, cycles: I, phase: f64) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.stimuli.push(StimulusSpec::Pulse {
            name: name.to_string(),
            cycles: cycles.into_iter().collect(),
            phase,
        });
        self
    }

    /// 初期値 0 から始まり、指定サイクルで反転する入力信号を指定する.
    ///
    /// 反転後の値は次の反転サイクルまで保持される.
    pub fn toggle<I>(mut self, name: &str, cycles: I, phase: f64) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        self.stimuli.push(StimulusSpec::Toggle {
            name: name.to_string(),
            cycles: cycles.into_iter().collect(),
            phase,
        });
        self
    }

    /// 入力信号以外に観察したい信号を指定する.
    ///
    /// 入力信号は自動的に観察対象へ含まれるので、ここには出力や内部ノードを書く.
    pub fn observe<N>(mut self, names: N) -> Self
    where
        N: IntoIterator,
        N::Item: AsRef<str>,
    {
        self.observe
            .extend(names.into_iter().map(|name| name.as_ref().to_string()));
        self
    }

    /// 指定した testbench backend でシミュレーション用コードを生成する.
    pub fn generate<B: TestbenchBackend>(&self, backend: B) -> String {
        backend.generate(self)
    }

    /// 指定した testbench backend の生成結果を標準出力へ出す.
    pub fn print<B: TestbenchBackend>(&self, backend: B) {
        println!("{}", self.generate(backend));
    }

    /// builder で受け取った複数形式の stimulus を backend 共通の形へ展開する.
    ///
    /// ここで入力ポートの過不足、重複指定、値域、位相、サイクル長をまとめて検証する.
    pub(crate) fn normalize(&self) -> NormalizedTestbench<'_> {
        let cycles = self
            .cycles
            .expect("testbench cycles must be specified before generation");
        let input_ports = self.circuit.in_ports();
        let input_set: BTreeSet<&str> = input_ports.iter().copied().collect();
        let mut signals = Vec::new();
        let mut specified_signals = BTreeSet::new();

        // 各 stimulus 指定を、信号名ごとの 0/1 ベクタへ正規化する.
        for stimulus in &self.stimuli {
            match stimulus {
                StimulusSpec::Signal {
                    name,
                    values,
                    phase,
                } => {
                    assert_input_name(name, &input_set);
                    insert_signal(
                        &mut signals,
                        &mut specified_signals,
                        SignalPattern {
                            name: name.clone(),
                            values: expand_signal(values, cycles),
                            phase: validate_phase(*phase),
                        },
                    );
                }
                StimulusSpec::Signals {
                    names,
                    values,
                    phase,
                } => {
                    assert!(!names.is_empty(), "signals must include at least one name");
                    for name in names {
                        assert_input_name(name, &input_set);
                    }
                    // signals は複数信号を一度に登録するが、重複検査は信号単位で行う.
                    let expanded = expand_signals(names, values, cycles);
                    let phase = validate_phase(*phase);
                    for (name, values) in expanded {
                        insert_signal(
                            &mut signals,
                            &mut specified_signals,
                            SignalPattern {
                                name,
                                values,
                                phase,
                            },
                        );
                    }
                }
                StimulusSpec::Constant { name, value, phase } => {
                    assert_input_name(name, &input_set);
                    let value = validate_bit(*value);
                    insert_signal(
                        &mut signals,
                        &mut specified_signals,
                        SignalPattern {
                            name: name.clone(),
                            values: vec![value; cycles],
                            phase: validate_phase(*phase),
                        },
                    );
                }
                StimulusSpec::Pulse {
                    name,
                    cycles: pulse_cycles,
                    phase,
                } => {
                    assert_input_name(name, &input_set);
                    let mut values = vec![0; cycles];
                    // pulse は指定されたサイクルだけ 1 にする疎な表現.
                    for cycle in pulse_cycles {
                        assert!(
                            *cycle < cycles,
                            "pulse cycle {} is outside testbench length {}",
                            cycle,
                            cycles
                        );
                        values[*cycle] = 1;
                    }
                    insert_signal(
                        &mut signals,
                        &mut specified_signals,
                        SignalPattern {
                            name: name.clone(),
                            values,
                            phase: validate_phase(*phase),
                        },
                    );
                }
                StimulusSpec::Toggle {
                    name,
                    cycles: toggle_cycles,
                    phase,
                } => {
                    assert_input_name(name, &input_set);
                    let toggle_set: BTreeSet<usize> = toggle_cycles.iter().copied().collect();
                    for cycle in &toggle_set {
                        assert!(
                            *cycle < cycles,
                            "toggle cycle {} is outside testbench length {}",
                            cycle,
                            cycles
                        );
                    }
                    let mut values = Vec::with_capacity(cycles);
                    let mut current = 0;
                    // toggle は「そのサイクル以降の状態」を反転するので、時系列に展開する.
                    for cycle in 0..cycles {
                        if toggle_set.contains(&cycle) {
                            current ^= 1;
                        }
                        values.push(current);
                    }
                    insert_signal(
                        &mut signals,
                        &mut specified_signals,
                        SignalPattern {
                            name: name.clone(),
                            values,
                            phase: validate_phase(*phase),
                        },
                    );
                }
            }
        }

        // 回路の全入力ポートについて、ちょうど一つの stimulus 指定が必要.
        for port in input_ports {
            assert!(
                specified_signals.contains(port),
                "input port `{}` has no testbench stimulus",
                port
            );
        }

        // 入力信号は自動的に observe へ追加し、ユーザー指定の observe を後ろに足す.
        let mut observe = Vec::new();
        let mut observed = BTreeSet::new();
        for signal in &signals {
            if observed.insert(signal.name.clone()) {
                observe.push(signal.name.clone());
            }
        }
        for name in &self.observe {
            if observed.insert(name.clone()) {
                observe.push(name.clone());
            }
        }

        NormalizedTestbench {
            name: self.circuit.name(),
            circuit: self.circuit,
            cycles,
            period_ps: self
                .period_ps
                .expect("testbench period_ps must be specified before generation"),
            signals,
            observe,
        }
    }
}

/// 任意の整数っぽい入力列を u64 列へ揃える.
///
/// 0/1 や bit 幅の検証は、用途ごとの展開関数で行う.
fn collect_numbers<I, V>(values: I) -> Vec<u64>
where
    I: IntoIterator<Item = V>,
    V: TryInto<u64>,
    V::Error: Debug,
{
    values
        .into_iter()
        .map(|value| value.try_into().expect("value must be convertible to u64"))
        .collect()
}

/// stimulus 名が対象回路の入力ポートであることを確認する.
fn assert_input_name(name: &str, input_set: &BTreeSet<&str>) {
    assert!(
        input_set.contains(name),
        "testbench stimulus `{}` is not a circuit input port",
        name
    );
}

/// 位相が 1 サイクル内の位置として有効か確認する.
fn validate_phase(phase: f64) -> f64 {
    assert!(
        phase.is_finite() && (0.0..1.0).contains(&phase),
        "testbench phase must satisfy 0.0 <= phase < 1.0"
    );
    phase
}

/// 1-bit stimulus の値を検証し、内部表現の u8 へ変換する.
fn validate_bit(value: u64) -> u8 {
    assert!(value <= 1, "1-bit stimulus value must be 0 or 1");
    value as u8
}

/// 正規化済み信号を追加し、同じ信号の二重指定を拒否する.
fn insert_signal(
    signals: &mut Vec<SignalPattern>,
    specified_signals: &mut BTreeSet<String>,
    signal: SignalPattern,
) {
    assert!(
        specified_signals.insert(signal.name.clone()),
        "testbench stimulus for `{}` is specified more than once",
        signal.name
    );
    signals.push(signal);
}

/// 単一信号の 0/1 列を `cycles` 長へ展開する.
fn expand_signal(values: &[u64], cycles: usize) -> Vec<u8> {
    assert!(
        values.len() <= cycles,
        "signal stimulus has {} values, but testbench has only {} cycles",
        values.len(),
        cycles
    );
    let mut expanded: Vec<u8> = values.iter().map(|value| validate_bit(*value)).collect();
    expanded.resize(cycles, 0);
    expanded
}

/// 整数列を MSB first の複数信号へ展開する.
fn expand_signals(names: &[String], values: &[u64], cycles: usize) -> Vec<(String, Vec<u8>)> {
    assert!(
        values.len() <= cycles,
        "signals stimulus has {} values, but testbench has only {} cycles",
        values.len(),
        cycles
    );
    let width = names.len();
    assert!(
        width < u64::BITS as usize,
        "signals width {} is too large for u64 values",
        width
    );
    let max_value = (1u64 << width) - 1;
    let mut expanded: Vec<(String, Vec<u8>)> = names
        .iter()
        .map(|name| (name.clone(), Vec::new()))
        .collect();

    for value in values {
        assert!(
            *value <= max_value,
            "signals value {} does not fit in {} bits",
            value,
            width
        );
        // names[index] が上位 bit に対応するよう、左から順に取り出す.
        for (index, (_, bits)) in expanded.iter_mut().enumerate() {
            let shift = width - 1 - index;
            bits.push(((value >> shift) & 1) as u8);
        }
    }

    for (_, bits) in &mut expanded {
        bits.resize(cycles, 0);
    }
    expanded
}
