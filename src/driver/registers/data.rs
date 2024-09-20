use bitfield::bitfield;

/// 9.4.1.3.1 状态字
///
/// 状态字 `[0:23]`
#[derive(Clone, Copy, Debug)]
pub struct DataStatus {
    pub ds1: DataStatus1,
    pub ds2: DataStatus2,
    pub ds3: DataStatus3,
}

bitfield! {
    /// 9.4.1.3.1 状态字
    ///
    /// 状态字 `[0:7]`
    #[derive(Clone, Copy)]
    pub struct DataStatus1(u8);
    impl Debug;
    bool;
    /// 通道 5 正通道导联脱落状态
    ///
    /// 有关 IN5P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in5p_off, set_in5p_off: 7;
    /// 通道 6 正通道导联脱落状态
    ///
    /// 有关 IN6P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in6p_off, set_in6p_off: 6;
    /// 通道 7 正通道导联脱落状态
    ///
    /// 有关 IN7P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in7p_off, set_in7p_off: 5;
    /// 通道 8 正通道导联脱落状态
    ///
    /// 有关 IN8P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in8p_off, set_in8p_off: 4;
    /// Should be `0`
    pub prefix_3, set_prefix_3: 3;
    /// Should be `0`
    pub prefix_2, set_prefix_2: 2;
    /// Should be `1`
    pub prefix_1, set_prefix_1: 1;
    /// Should be `1`
    pub prefix_0, set_prefix_0: 0;
}

bitfield! {
    /// 9.4.1.3.1 状态字
    ///
    /// 状态字 `[8:15]`
    #[derive(Clone, Copy)]
    pub struct DataStatus2(u8);
    impl Debug;
    bool;
    /// 通道 5 负通道导联脱落状态
    ///
    /// 有关 IN5N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in5n_off, set_in5n_off: 7;
    /// 通道 6 负通道导联脱落状态
    ///
    /// 有关 IN6N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in6n_off, set_in6n_off: 6;
    /// 通道 7 负通道导联脱落状态
    ///
    /// 有关 IN7N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in7n_off, set_in7n_off: 5;
    /// 通道 8 负通道导联脱落状态
    ///
    /// 有关 IN8N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in8n_off, set_in8n_off: 4;
    /// 通道 1 正通道导联脱落状态
    ///
    /// 有关 IN1P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in1p_off, set_in1p_off: 3;
    /// 通道 2 正通道导联脱落状态
    ///
    /// 有关 IN2P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in2p_off, set_in2p_off: 2;
    /// 通道 3 正通道导联脱落状态
    ///
    /// 有关 IN3P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in3p_off, set_in3p_off: 1;
    /// 通道 4 正通道导联脱落状态
    ///
    /// 有关 IN4P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in4p_off, set_in4p_off: 0;
}

bitfield! {
    /// 9.4.1.3.1 状态字
    ///
    /// 状态字 `[16:23]`
    #[derive(Clone, Copy)]
    pub struct DataStatus3(u8);
    impl Debug;
    bool;
    /// GPIO 数据 `[1]`
    ///
    /// 这些位用于从 GPIO 端口读取数据以及向其中写入数据。
    /// 在读取寄存器时，返回的数据对应于 GPIO 外部引脚的状态，无
    /// 论它们是编程为输入还是输出都是如此。作为输出时，对 GPIOD
    /// 进行写入可设置输出值。作为输入时，对 GPIOD 进行写入无效。
    /// GPIO 在某些呼吸模式下不可用。
    pub gpiod_1, set_gpiod_1: 7;
    /// GPIO 数据 `[2]`
    ///
    /// 这些位用于从 GPIO 端口读取数据以及向其中写入数据。
    /// 在读取寄存器时，返回的数据对应于 GPIO 外部引脚的状态，无
    /// 论它们是编程为输入还是输出都是如此。作为输出时，对 GPIOD
    /// 进行写入可设置输出值。作为输入时，对 GPIOD 进行写入无效。
    /// GPIO 在某些呼吸模式下不可用。
    pub gpiod_2, set_gpiod_2: 6;
    /// GPIO 数据 `[3]`
    ///
    /// 这些位用于从 GPIO 端口读取数据以及向其中写入数据。
    /// 在读取寄存器时，返回的数据对应于 GPIO 外部引脚的状态，无
    /// 论它们是编程为输入还是输出都是如此。作为输出时，对 GPIOD
    /// 进行写入可设置输出值。作为输入时，对 GPIOD 进行写入无效。
    /// GPIO 在某些呼吸模式下不可用。
    pub gpiod_3, set_gpiod_3: 5;
    /// GPIO 数据 `[4]`
    ///
    /// 这些位用于从 GPIO 端口读取数据以及向其中写入数据。
    /// 在读取寄存器时，返回的数据对应于 GPIO 外部引脚的状态，无
    /// 论它们是编程为输入还是输出都是如此。作为输出时，对 GPIOD
    /// 进行写入可设置输出值。作为输入时，对 GPIOD 进行写入无效。
    /// GPIO 在某些呼吸模式下不可用。
    pub gpiod_4, set_gpiod_4: 4;
    /// 通道 1 负通道导联脱落状态
    ///
    /// 有关 IN1N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in1n_off, set_in1n_off: 3;
    /// 通道 2 负通道导联脱落状态
    ///
    /// 有关 IN2N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in2n_off, set_in2n_off: 2;
    /// 通道 3 负通道导联脱落状态
    ///
    /// 有关 IN3N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in3n_off, set_in3n_off: 1;
    /// 通道 4 负通道导联脱落状态
    ///
    /// 有关 IN4N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in4n_off, set_in4n_off: 0;
}

bitfield! {
    /// ID 控制寄存器 地址 = `00h` 复位 = `xxh`
    pub struct IdReg(u8);
    impl Debug;
    bool;
    /// 器件 ID `[7]` `R` 复位 = `X`
    dev_id_7, set_dev_id_7: 7;
    /// 器件 ID `[6]` `R` 复位 = `X`
    dev_id_6, set_dev_id_6: 6;
    /// 器件 ID `[5]` `R` 复位 = `X`
    dev_id_5, set_dev_id_5: 5;
    /// 保留 `[4]` `R` 复位 = `1`
    rev_4, set_rev_4: 4;
    /// 保留 `[3]` `R` 复位 = `0`
    rev_3, set_rev_3: 3;
    /// 通道 ID `[2]` `R` 复位 = `X`
    dev_id_2, set_dev_id_2: 2;
    /// 通道 ID `[1]` `R` 复位 = `X`
    dev_id_1, set_dev_id_1: 1;
    /// 通道 ID `[0]` `R` 复位 = `X`
    dev_id_0, set_dev_id_0: 0;
}

bitfield! {
    /// 配置寄存器 1 地址 = `01h` 复位 = `06h`
    pub struct Config1Reg(u8);
    impl Debug;
    bool; u8;
    /// 高分辨率或低功耗模式 `[7]` `R/W` 复位 = `0`
    ///
    /// 该位决定器件是以低功耗模式运行还是以高分辨率模式运行。
    ///
    /// 0 = LP 模式, 1 = HR 模式
    hr, set_hr: 7;
    /// 菊花链或多读回模式 `[6]` `R/W` 复位 = `0`
    ///
    /// 该位决定启用哪种模式。
    ///
    /// 0 = 菊花链模式, 1 = 多读回模式
    daisy_en, set_daisy_en: 6;
    /// CLK 连接 `[5]` `R/W` 复位 = `0`
    ///
    /// 该位决定当 CLKSEL 引脚 = 1 时内部振荡器信号是否连接到 CLK
    /// 引脚。
    ///
    /// - 0 = 禁用振荡器时钟输出
    /// - 1 = 启用振荡器时钟输出
    clk_en, set_clk_en: 5;
    /// 保留 `[4]` `R/W` 复位 = `0`
    rev_4, set_rev_4: 4;
    /// 保留 `[3]` `R/W` 复位 = `0`
    rev_3, set_rev_3: 3;
    /// 输出数据速率 `[5]` `R/W` 复位 = `110b = 6`
    ///
    /// 对于高分辨率模式，fMOD = fCLK/4。低于低功耗模式，fMOD =
    /// fCLK/8。
    ///
    /// 这些位决定器件的输出数据速率。
    ///
    /// - 000：fMOD/16（HR 模式：32kSPS，LP 模式：16kSPS）
    /// - 001：fMOD/32（HR 模式：16kSPS，LP 模式：8kSPS）
    /// - 010：fMOD/64（HR 模式：8kSPS，LP 模式：4kSPS）
    /// - 011：fMOD/128（HR 模式：4kSPS，LP 模式：2kSPS）
    /// - 100：fMOD/256（HR 模式：2kSPS，LP 模式：1kSPS）
    /// - 101：fMOD/512（HR 模式：1kSPS，LP 模式：500SPS）
    /// - 110：fMOD/1024（HR 模式：500SPS，LP 模式：250SPS）
    /// - 111：保留（不使用）
    dr, set_dr: 2, 0;
}

bitfield! {
    /// 配置寄存器 2 地址 = `02h` 复位 = `40h`
    pub struct Config2Reg(u8);
    impl Debug;
    bool; u8;
    /// 保留 `[7]` `R/W` 复位 = `0`
    /// 
    /// 始终写入 `0`
    rev_7, set_rev_7: 7;
    /// 保留 `[6]` `R/W` 复位 = `1`
    /// 
    /// 始终写入 `0`
    rev_6, set_rev_6: 6;
    /// WCT 斩波方案 `[5]` `R/W` 复位 = `0`
    ///
    /// 该位决定 WCT 放大器的斩波频率是可变还是固定的。
    ///
    /// - 0 = 斩波频率可变，请参阅 Table 7
    /// - 1 = 斩波频率恒定保持在 fMOD/16
    wct_chop, set_wct_chop: 5;
    /// 测试源 `[4]` `R/W` 复位 = `0`
    ///
    /// 该位决定测试信号源。
    ///
    /// - 0 = 从外部驱动测试信号
    /// - 1 = 在内部生成测试信号
    int_test, set_int_test: 4;
    /// 保留 `[3]` `R/W` 复位 = `0`
    rev_3, set_rev_3: 3;
    /// 测试信号振幅 `[2]` `R/W` 复位 = `0`
    ///
    /// 这些位决定校准信号振幅。
    ///
    /// - 0 = 1 × –(VREFP – VREFN)/2400V
    /// - 1 = 2 × –(VREFP – VREFN)/2400V
    test_amp, set_test_amp: 2;
    /// 测试信号频率 `[1:0]` `R/W` 复位 = `0`
    ///
    /// 这些位决定校准信号频率。
    ///
    /// - 00 = 以 fCLK/2^21 的频率发送脉冲信号
    /// - 01 = 以 fCLK/2^20 的频率发送脉冲信号
    /// - 10 = 未使用
    /// - 11 = 直流
    test_freq, set_test_freq: 1, 0;
}

bitfield! {
    /// 配置寄存器 3 地址 = `03h` 复位 = `40h`
    pub struct Config3Reg(u8);
    impl Debug;
    bool;
    /// 关断基准缓冲器 `[7]` `R/W` 复位 = `0`
    ///
    /// 该位决定关断基准缓冲器状态。
    ///
    /// - 0 = 关断内部基准缓冲器
    /// - 1 = 启用内部基准缓冲器
    pd_refbuf, set_pd_refbuf: 7;
    /// 保留 `[6]` `R/W` 复位 = `1`
    /// 
    /// 始终写入 `1`
    rev_6, set_rev_6: 6;
    /// 基准电压 `[5]` `R/W` 复位 = `0`
    /// 
    /// 该位决定基准电压 VREFP。
    /// 
    /// - 0 = VREFP 设置为 2.4V
    /// - 1 = VREFP 设置为 4V（仅与 5V 模拟电源配合使用）
    vref_4v, set_vref_4v: 5;
    /// RLD 测量 `[4]` `R/W` 复位 = `0`
    ///
    /// 该位启用 RLD 测量。可以使用任何通道测量 RLD 信号。
    ///
    /// - 0 = 开路
    /// - 1 = RLD_IN 信号路由至具有 MUX_Setting 010 (VREF) 的通道
    rld_meas, set_rld_meas: 4;
    /// RLD 缓冲器电源 `[3]` `R/W` 复位 = `0`
    /// 
    /// 该位决定 RLD 缓冲器电源状态。
    /// 
    /// - 0 = RLD 缓冲器断电
    /// - 1 = 启用 RLD 缓冲器
    pd_rld, set_pd_rld: 3;
    /// RLD 感应功能 `[2]` `R/W` 复位 = `0`
    ///
    /// 该位启用 RLD 感应功能。
    ///
    /// - 0 = 禁用 RLD 感应
    /// - 1 = 启用 RLD 感应
    rld_loff_sens, set_rld_loff_sens: 2;

}
