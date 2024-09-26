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
    pub in5p_off, set_in5p_off: 0;
    /// 通道 6 正通道导联脱落状态
    ///
    /// 有关 IN6P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in6p_off, set_in6p_off: 1;
    /// 通道 7 正通道导联脱落状态
    ///
    /// 有关 IN7P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in7p_off, set_in7p_off: 2;
    /// 通道 8 正通道导联脱落状态
    ///
    /// 有关 IN8P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in8p_off, set_in8p_off: 3;
    /// Should be `0`
    pub prefix_4, set_prefix_4: 4;
    /// Should be `0`
    pub prefix_5, set_prefix_5: 5;
    /// Should be `1`
    pub prefix_6, set_prefix_6: 6;
    /// Should be `1`
    pub prefix_7, set_prefix_7: 7;
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
    pub in5n_off, set_in5n_off: 0;
    /// 通道 6 负通道导联脱落状态
    ///
    /// 有关 IN6N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in6n_off, set_in6n_off: 1;
    /// 通道 7 负通道导联脱落状态
    ///
    /// 有关 IN7N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in7n_off, set_in7n_off: 2;
    /// 通道 8 负通道导联脱落状态
    ///
    /// 有关 IN8N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in8n_off, set_in8n_off: 3;
    /// 通道 1 正通道导联脱落状态
    ///
    /// 有关 IN1P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in1p_off, set_in1p_off: 4;
    /// 通道 2 正通道导联脱落状态
    ///
    /// 有关 IN2P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in2p_off, set_in2p_off: 5;
    /// 通道 3 正通道导联脱落状态
    ///
    /// 有关 IN3P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in3p_off, set_in3p_off: 6;
    /// 通道 4 正通道导联脱落状态
    ///
    /// 有关 IN4P 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in4p_off, set_in4p_off: 7;
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
    pub gpiod_1, set_gpiod_1: 0;
    /// GPIO 数据 `[2]`
    ///
    /// 这些位用于从 GPIO 端口读取数据以及向其中写入数据。
    /// 在读取寄存器时，返回的数据对应于 GPIO 外部引脚的状态，无
    /// 论它们是编程为输入还是输出都是如此。作为输出时，对 GPIOD
    /// 进行写入可设置输出值。作为输入时，对 GPIOD 进行写入无效。
    /// GPIO 在某些呼吸模式下不可用。
    pub gpiod_2, set_gpiod_2: 1;
    /// GPIO 数据 `[3]`
    ///
    /// 这些位用于从 GPIO 端口读取数据以及向其中写入数据。
    /// 在读取寄存器时，返回的数据对应于 GPIO 外部引脚的状态，无
    /// 论它们是编程为输入还是输出都是如此。作为输出时，对 GPIOD
    /// 进行写入可设置输出值。作为输入时，对 GPIOD 进行写入无效。
    /// GPIO 在某些呼吸模式下不可用。
    pub gpiod_3, set_gpiod_3: 2;
    /// GPIO 数据 `[4]`
    ///
    /// 这些位用于从 GPIO 端口读取数据以及向其中写入数据。
    /// 在读取寄存器时，返回的数据对应于 GPIO 外部引脚的状态，无
    /// 论它们是编程为输入还是输出都是如此。作为输出时，对 GPIOD
    /// 进行写入可设置输出值。作为输入时，对 GPIOD 进行写入无效。
    /// GPIO 在某些呼吸模式下不可用。
    pub gpiod_4, set_gpiod_4: 3;
    /// 通道 1 负通道导联脱落状态
    ///
    /// 有关 IN1N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in1n_off, set_in1n_off: 4;
    /// 通道 2 负通道导联脱落状态
    ///
    /// 有关 IN2N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in2n_off, set_in2n_off: 5;
    /// 通道 3 负通道导联脱落状态
    ///
    /// 有关 IN3N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in3n_off, set_in3n_off: 6;
    /// 通道 4 负通道导联脱落状态
    ///
    /// 有关 IN4N 电极是打开还是关闭的状态
    ///
    /// 0：电极打开, 1：电极关闭
    pub in4n_off, set_in4n_off: 7;
}

bitfield! {
    /// ID 控制寄存器 地址 = `00h` 复位 = `xxh`
    #[derive(Clone, Copy)]
    pub struct IdReg(u8);
    impl Debug;
    bool;
    /// 器件 ID `[7]` `R` 复位 = `X`
    pub dev_id_7, set_dev_id_7: 7;
    /// 器件 ID `[6]` `R` 复位 = `X`
    pub dev_id_6, set_dev_id_6: 6;
    /// 器件 ID `[5]` `R` 复位 = `X`
    pub dev_id_5, set_dev_id_5: 5;
    /// 保留 `[4]` `R` 复位 = `1`
    pub rev_4, set_rev_4: 4;
    /// 保留 `[3]` `R` 复位 = `0`
    pub rev_3, set_rev_3: 3;
    /// 通道 ID `[2]` `R` 复位 = `X`
    pub dev_id_2, set_dev_id_2: 2;
    /// 通道 ID `[1]` `R` 复位 = `X`
    pub dev_id_1, set_dev_id_1: 1;
    /// 通道 ID `[0]` `R` 复位 = `X`
    pub dev_id_0, set_dev_id_0: 0;
}

bitfield! {
    /// 配置寄存器 1 地址 = `01h` 复位 = `06h`
    #[derive(Clone, Copy)]
    pub struct Config1Reg(u8);
    impl Debug;
    bool; u8;
    /// 高分辨率或低功耗模式 `[7]` `R/W` 复位 = `0`
    ///
    /// 该位决定器件是以低功耗模式运行还是以高分辨率模式运行。
    ///
    /// - 0 = LP 模式
    /// - 1 = HR 模式
    pub hr, set_hr: 7;
    /// 菊花链或多读回模式 `[6]` `R/W` 复位 = `0`
    ///
    /// 该位决定启用哪种模式。
    ///
    /// - 0 = 菊花链模式
    /// - 1 = 多读回模式
    pub daisy_en, set_daisy_en: 6;
    /// CLK 连接 `[5]` `R/W` 复位 = `0`
    ///
    /// 该位决定当 CLKSEL 引脚 = 1 时内部振荡器信号是否连接到 CLK
    /// 引脚。
    ///
    /// - 0 = 禁用振荡器时钟输出
    /// - 1 = 启用振荡器时钟输出
    pub clk_en, set_clk_en: 5;
    /// 保留 `[4]` `R/W` 复位 = `0`
    pub rev_4, set_rev_4: 4;
    /// 保留 `[3]` `R/W` 复位 = `0`
    pub rev_3, set_rev_3: 3;
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
    pub dr, set_dr: 2, 0;
}

bitfield! {
    /// 配置寄存器 2 地址 = `02h` 复位 = `40h`
    #[derive(Clone, Copy)]
    pub struct Config2Reg(u8);
    impl Debug;
    bool; u8;
    /// 保留 `[7]` `R/W` 复位 = `0`
    ///
    /// 始终写入 `0`
    pub rev_7, set_rev_7: 7;
    /// 保留 `[6]` `R/W` 复位 = `1`
    ///
    /// 始终写入 `0`
    pub rev_6, set_rev_6: 6;
    /// WCT 斩波方案 `[5]` `R/W` 复位 = `0`
    ///
    /// 该位决定 WCT 放大器的斩波频率是可变还是固定的。
    ///
    /// - 0 = 斩波频率可变，请参阅 Table 7
    /// - 1 = 斩波频率恒定保持在 fMOD/16
    pub wct_chop, set_wct_chop: 5;
    /// 测试源 `[4]` `R/W` 复位 = `0`
    ///
    /// 该位决定测试信号源。
    ///
    /// - 0 = 从外部驱动测试信号
    /// - 1 = 在内部生成测试信号
    pub int_test, set_int_test: 4;
    /// 保留 `[3]` `R/W` 复位 = `0`
    ///
    /// 始终写入 `0`
    pub rev_3, set_rev_3: 3;
    /// 测试信号振幅 `[2]` `R/W` 复位 = `0`
    ///
    /// 这些位决定校准信号振幅。
    ///
    /// - 0 = 1 × –(VREFP – VREFN)/2400V
    /// - 1 = 2 × –(VREFP – VREFN)/2400V
    pub test_amp, set_test_amp: 2;
    /// 测试信号频率 `[1:0]` `R/W` 复位 = `0`
    ///
    /// 这些位决定校准信号频率。
    ///
    /// - 00 = 以 fCLK/2^21 的频率发送脉冲信号
    /// - 01 = 以 fCLK/2^20 的频率发送脉冲信号
    /// - 10 = 未使用
    /// - 11 = 直流
    pub test_freq, set_test_freq: 1, 0;
}

bitfield! {
    /// 配置寄存器 3 地址 = `03h` 复位 = `40h`
    #[derive(Clone, Copy)]
    pub struct Config3Reg(u8);
    impl Debug;
    bool;
    /// 关断基准缓冲器 `[7]` `R/W` 复位 = `0`
    ///
    /// 该位决定关断基准缓冲器状态。
    ///
    /// - 0 = 关断内部基准缓冲器
    /// - 1 = 启用内部基准缓冲器
    pub pd_refbuf, set_pd_refbuf: 7;
    /// 保留 `[6]` `R/W` 复位 = `1`
    ///
    /// 始终写入 `1`
    pub rev_6, set_rev_6: 6;
    /// 基准电压 `[5]` `R/W` 复位 = `0`
    ///
    /// 该位决定基准电压 VREFP。
    ///
    /// - 0 = VREFP 设置为 2.4V
    /// - 1 = VREFP 设置为 4V（仅与 5V 模拟电源配合使用）
    pub vref_4v, set_vref_4v: 5;
    /// RLD 测量 `[4]` `R/W` 复位 = `0`
    ///
    /// 该位启用 RLD 测量。可以使用任何通道测量 RLD 信号。
    ///
    /// - 0 = 开路
    /// - 1 = RLD_IN 信号路由至具有 MUX_Setting 010 (VREF) 的通道
    pub rld_meas, set_rld_meas: 4;
    /// RLDREF 信号 `[3]` `R/W` 复位 = `0`
    ///
    /// 该位决定 RLDREF 信号源。
    ///
    /// - 0 = 从外部馈送 RLDREF 信号
    /// - 1 = 在内部生成 RLDREF 信号 (AVDD – AVSS)/2
    pub rldref_int, set_rldref_int: 3;
    /// RLD 缓冲器电源 `[2]` `R/W` 复位 = `0`
    ///
    /// 该位决定 RLD 缓冲器电源状态。
    ///
    /// - 0 = RLD 缓冲器断电
    /// - 1 = 启用 RLD 缓冲器
    pub pd_rld, set_pd_rld: 2;
    /// RLD 感应功能 `[1]` `R/W` 复位 = `0`
    ///
    /// 该位启用 RLD 感应功能。
    ///
    /// - 0 = 禁用 RLD 感应
    /// - 1 = 启用 RLD 感应
    pub rld_loff_sens, set_rld_loff_sens: 1;
    /// RLD 导联脱落状态 `[0]` `R` 复位 = `0`
    ///
    /// 该位决定 RLD 状态。
    ///
    /// - 0 = RLD 已连接
    /// - 1 = RLD 未连接
    pub rld_stat, set_rld_stat: 0;
}

bitfield! {
    /// 导联脱落控制寄存器 地址 = `04h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct LOffReg(u8);
    impl Debug;
    bool; u8;
    /// 导联脱落比较器阈值 `[7:5]` `R/W` 复位 = `0h`
    ///
    /// 比较器正极侧
    ///
    /// - 000 = 95%
    /// - 001 = 92.5%
    /// - 010 = 90%
    /// - 011 = 87.5%
    /// - 100 = 85%
    /// - 101 = 80%
    /// - 110 = 75%
    /// - 111 = 70%
    ///
    /// 比较器负极侧
    ///
    /// - 000 = 5%
    /// - 001 = 7.5%
    /// - 010 = 10%
    /// - 011 = 12.5%
    /// - 100 = 15%
    /// - 101 = 20%
    /// - 110 = 25%
    /// - 111 = 30%
    pub comp_th, set_comp_th: 7, 5;
    /// 导联脱落检测模式 `[4]` `R/W` 复位 = `0h`
    ///
    /// 该位决定导联脱落检测模式。
    ///
    /// - 0 = 电流源模式导联脱落
    /// - 1 = 上拉或下拉电阻器模式导联脱落
    pub vlead_off_en, set_vlead_off_en: 4;
    /// 导联脱落电流幅度 `[3:2]` `R/W` 复位 = `0h`
    ///
    /// 这些位决定当前导联脱落模式的电流幅度。
    ///
    /// - 00 = 6nA
    /// - 01 = 12nA
    /// - 10 = 18nA
    /// - 11 = 24nA
    pub ilead_off, set_ilead_off: 3, 2;
    /// 导联脱落频率 `[1:0]` `R/W` 复位 = `0h`
    ///
    /// 这些位决定每个通道的导联脱落检测的频率。
    ///
    /// - 00 = 当 LOFF_SENSP 或 LOFF_SENSN 寄存器的任何位打开时，确保 FLEAD[1:0] 设置为 01 或 11
    /// - 01 = 以 FDR/4 的频率执行交流导联脱落检测
    /// - 10 = 不使用
    /// - 11 = 直流导联脱落检测打开
    pub flead_off, set_flead_off: 1, 0;
}

bitfield! {
    /// 通道设置 地址 = `05h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct ChSetReg(u8);
    impl Debug;
    bool; u8;
    /// 断电 `[7]` `R/W` 复位 = `0`
    ///
    /// 该位决定相应通道的通道电源模式。
    ///
    /// - 0 = 正常运行
    /// - 1 = 通道断电
    ///
    /// 在关闭某个通道的电源时，TI 建议通过设置 CHnSET 寄存器的相应
    /// MUXn`[2:0]` = 001，将该通道设置为输入短路。
    pub pd, set_pd: 7;
    /// PGA 增益 `[6:4]` `R/W` 复位 = `0`
    ///
    /// 这些位决定 PGA 增益设置。
    ///
    /// - 000 = 6
    /// - 001 = 1
    /// - 010 = 2
    /// - 011 = 3
    /// - 100 = 4
    /// - 101 = 8
    /// - 110 = 12
    pub gain, set_gain: 6, 4;
    /// 保留 `[3]` `R/W` 复位 = `0`
    ///
    /// 始终写入 `0`
    pub rev_3, set_rev_3: 3;
    /// 通道输入 `[2:0]` `R/W` 复位 = `0`
    ///
    /// 这些位决定通道输入选择。
    ///
    /// - 000 = 正常电极输入
    /// - 001 = 输入短路（对于偏移或噪声测量）
    /// - 010 = 与 RLD_MEAS 位结合使用，以进行 RLD 测量。有关更多详细信息，请参阅
    /// ECG 专用功能 部分的右腿驱动 (RLD) 直流偏置电流 小节。
    /// - 011 = MVDD，用于电源测量
    /// - 100 = 温度传感器
    /// - 101 = 测试信号
    /// - 110 = RLD_DRP（正电极是驱动器）
    /// - 111 = RLD_DRN（负电极是驱动器）
    pub mux, set_mux: 2, 0;
}

bitfield! {
    /// RLD 正信号导出寄存器 地址 = `0Dh` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct RldSensPReg(u8);
    impl Debug;
    bool; u8;
    /// 将通道 8 正信号路由到 RLD 导出中 `[7]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld8p, set_rld8p: 7;
    /// 将通道 7 正信号路由到 RLD 导出中 `[6]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld7p, set_rld7p: 6;
    /// 将通道 6 正信号路由到 RLD 导出中 `[5]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld6p, set_rld6p: 5;
    /// 将通道 5 正信号路由到 RLD 导出中 `[4]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld5p, set_rld5p: 4;
    /// 将通道 4 正信号路由到 RLD 导出中 `[3]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld4p, set_rld4p: 3;
    /// 将通道 3 正信号路由到 RLD 导出中 `[2]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld3p, set_rld3p: 2;
    /// 将通道 2 正信号路由到 RLD 通道中 `[1]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld2p, set_rld2p: 1;
    /// 将通道 1 正信号路由到 RLD 通道中 `[0]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld1p, set_rld1p: 0;
}

bitfield! {
    /// RLD 负信号导出寄存器 地址 = `0Eh` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct RldSensNReg(u8);
    impl Debug;
    bool; u8;
    /// 将通道 8 负信号路由到 RLD 导出中 `[7]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld8n, set_rld8n: 7;
    /// 将通道 7 负信号路由到 RLD 导出中 `[6]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld7n, set_rld7n: 6;
    /// 将通道 6 负信号路由到 RLD 导出中 `[5]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld6n, set_rld6n: 5;
    /// 将通道 5 负信号路由到 RLD 导出中 `[4]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld5n, set_rld5n: 4;
    /// 将通道 4 负信号路由到 RLD 导出中 `[3]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld4n, set_rld4n: 3;
    /// 将通道 3 负信号路由到 RLD 导出中 `[2]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld3n, set_rld3n: 2;
    /// 将通道 2 负信号路由到 RLD 导出中 `[1]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld2n, set_rld2n: 1;
    /// 将通道 1 负信号路由到 RLD 导出中 `[0]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub rld1n, set_rld1n: 0;
}

bitfield! {
    /// 正信号导联脱落检测寄存器 地址 = `0Fh` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct LOffSensPReg(u8);
    impl Debug;
    bool; u8;
    /// 启用 IN8P 上的导联脱落检测 `[7]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff8p, set_loff8p: 7;
    /// 启用 IN7P 上的导联脱落检测 `[6]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff7p, set_loff7p: 6;
    /// 启用 IN6P 上的导联脱落检测 `[5]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff6p, set_loff6p: 5;
    /// 启用 IN5P 上的导联脱落检测 `[4]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff5p, set_loff5p: 4;
    /// 启用 IN4P 上的导联脱落检测 `[3]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff4p, set_loff4p: 3;
    /// 启用 IN3P 上的导联脱落检测 `[2]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff3p, set_loff3p: 2;
    /// 启用 IN2P 上的导联脱落检测 `[1]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff2p, set_loff2p: 1;
    /// 启用 IN1P 上的导联脱落检测 `[0]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff1p, set_loff1p: 0;
}

bitfield! {
    /// 负信号导联脱落检测寄存器 地址 = `10h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct LOffSensNReg(u8);
    impl Debug;
    bool; u8;
    /// 启用 IN8N 上的导联脱落检测 `[7]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff8n, set_loff8n: 7;
    /// 启用 IN7N 上的导联脱落检测 `[6]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff7n, set_loff7n: 6;
    /// 启用 IN6N 上的导联脱落检测 `[5]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff6n, set_loff6n: 5;
    /// 启用 IN5N 上的导联脱落检测 `[4]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff5n, set_loff5n: 4;
    /// 启用 IN4N 上的导联脱落检测 `[3]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff4n, set_loff4n: 3;
    /// 启用 IN3N 上的导联脱落检测 `[2]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff3n, set_loff3n: 2;
    /// 启用 IN2N 上的导联脱落检测 `[1]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff2n, set_loff2n: 1;
    /// 启用 IN1N 上的导联脱落检测 `[0]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub loff1n, set_loff1n: 0;
}

bitfield! {
    /// 导联脱落翻转寄存器 地址 = `11h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct LoffFlipReg(u8);
    impl Debug;
    bool; u8;
    /// 通道 8 LOFF 极性翻转 `[7]` `R/W` 复位 = `0`
    ///
    /// 翻转通道 8 上电流源或电阻器的上拉/下拉极性（用于导联脱落导出）。
    ///
    /// - 0 = 无翻转：IN8P 拉至 AVDD，IN8N 拉至 AVSS
    /// - 1 = 翻转：IN8P 拉至 AVSS，IN8N 拉至 AVDD
    pub loff_flip8, set_loff_flip8: 7;
    /// 通道 7 LOFF 极性翻转 `[6]` `R/W` 复位 = `0`
    ///
    /// 翻转通道 7 上电流源或电阻器的上拉/下拉极性（用于导联脱落导出）。
    ///
    /// - 0 = 无翻转：IN7P 拉至 AVDD，IN7N 拉至 AVSS
    /// - 1 = 翻转：IN7P 拉至 AVSS，IN7N 拉至 AVDD
    pub loff_flip7, set_loff_flip7: 6;
    /// 通道 6 LOFF 极性翻转 `[5]` `R/W` 复位 = `0`
    ///
    /// 翻转通道 6 上电流源或电阻器的上拉/下拉极性（用于导联脱落导出）。
    ///
    /// - 0 = 无翻转：IN6P 拉至 AVDD，IN6N 拉至 AVSS
    /// - 1 = 翻转：IN6P 拉至 AVSS，IN6N 拉至 AVDD
    pub loff_flip6, set_loff_flip6: 5;
    /// 通道 5 LOFF 极性翻转 `[4]` `R/W` 复位 = `0`
    ///
    /// 翻转通道 5 上电流源或电阻器的上拉/下拉极性（用于导联脱落导出）。
    ///
    /// - 0 = 无翻转：IN5P 拉至 AVDD，IN5N 拉至 AVSS
    /// - 1 = 翻转：IN5P 拉至 AVSS，IN5N 拉至 AVDD
    pub loff_flip5, set_loff_flip5: 4;
    /// 通道 4 LOFF 极性翻转 `[3]` `R/W` 复位 = `0`
    ///
    /// 翻转通道 4 上电流源或电阻器的上拉/下拉极性（用于导联脱落导出）。
    ///
    /// - 0 = 无翻转：IN4P 拉至 AVDD，IN4N 拉至 AVSS
    /// - 1 = 翻转：IN4P 拉至 AVSS，IN4N 拉至 AVDD
    pub loff_flip4, set_loff_flip4: 3;
    /// 通道 3 LOFF 极性翻转 `[2]` `R/W` 复位 = `0`
    ///
    /// 翻转通道 3 上电流源或电阻器的上拉/下拉极性（用于导联脱落导出）。
    ///
    /// - 0 = 无翻转：IN3P 拉至 AVDD，IN3N 拉至 AVSS
    /// - 1 = 翻转：IN3P 拉至 AVSS，IN3N 拉至 AVDD
    pub loff_flip3, set_loff_flip3: 2;
    /// 通道 2 LOFF 极性翻转 `[1]` `R/W` 复位 = `0`
    ///
    /// 翻转通道 2 上电流源或电阻器的上拉/下拉极性（用于导联脱落导出）。
    ///
    /// - 0 = 无翻转：IN2P 拉至 AVDD，IN2N 拉至 AVSS
    /// - 1 = 翻转：IN2P 拉至 AVSS，IN2N 拉至 AVDD
    pub loff_flip2, set_loff_flip2: 1;
    /// 通道 1 LOFF 极性翻转 `[0]` `R/W` 复位 = `0`
    ///
    /// 翻转通道 1 上电流源或电阻器的上拉/下拉极性（用于导联脱落导出）。
    ///
    /// - 0 = 无翻转：IN1P 拉至 AVDD，IN1N 拉至 AVSS
    /// - 1 = 翻转：IN1P 拉至 AVSS，IN1N 拉至 AVDD
    pub loff_flip1, set_loff_flip1: 0;
}

bitfield! {
    /// 导联脱落正信号状态寄存器 地址 = `12h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct LOffStatPReg(u8);
    impl Debug;
    bool; u8;
    /// 通道 8 正通道导联脱落状态 `[7]` `R` 复位 = `0`
    ///
    /// 有关 IN8P 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in8p_off, set_in8p_off: 7;
    /// 通道 7 正通道导联脱落状态 `[6]` `R` 复位 = `0`
    ///
    /// 有关 IN7P 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in7p_off, set_in7p_off: 6;
    /// 通道 6 正通道导联脱落状态 `[5]` `R` 复位 = `0`
    ///
    /// 有关 IN6P 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in6p_off, set_in6p_off: 5;
    /// 通道 5 正通道导联脱落状态 `[4]` `R` 复位 = `0`
    ///
    /// 有关 IN5P 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in5p_off, set_in5p_off: 4;
    /// 通道 4 正通道导联脱落状态 `[3]` `R` 复位 = `0`
    ///
    /// 有关 IN4P 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in4p_off, set_in4p_off: 3;
    /// 通道 3 正通道导联脱落状态 `[2]` `R` 复位 = `0`
    ///
    /// 有关 IN3P 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in3p_off, set_in3p_off: 2;
    /// 通道 2 正通道导联脱落状态 `[1]` `R` 复位 = `0`
    ///
    /// 有关 IN2P 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in2p_off, set_in2p_off: 1;
    /// 通道 1 正通道导联脱落状态 `[0]` `R` 复位 = `0`
    ///
    /// 有关 IN1P 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in1p_off, set_in1p_off: 0;
}

bitfield! {
    /// 导联脱落负信号状态寄存器 地址 = `13h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct LOffStatNReg(u8);
    impl Debug;
    bool; u8;
    /// 通道 8 负通道导联脱落状态 `[7]` `R` 复位 = `0`
    ///
    /// 有关 IN8N 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in8n_off, set_in8n_off: 7;
    /// 通道 7 负通道导联脱落状态 `[6]` `R` 复位 = `0`
    ///
    /// 有关 IN7N 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in7n_off, set_in7n_off: 6;
    /// 通道 6 负通道导联脱落状态 `[5]` `R` 复位 = `0`
    ///
    /// 有关 IN6N 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in6n_off, set_in6n_off: 5;
    /// 通道 5 负通道导联脱落状态 `[4]` `R` 复位 = `0`
    ///
    /// 有关 IN5N 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in5n_off, set_in5n_off: 4;
    /// 通道 4 负通道导联脱落状态 `[3]` `R` 复位 = `0`
    ///
    /// 有关 IN4N 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in4n_off, set_in4n_off: 3;
    /// 通道 3 负通道导联脱落状态 `[2]` `R` 复位 = `0`
    ///
    /// 有关 IN3N 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in3n_off, set_in3n_off: 2;
    /// 通道 2 负通道导联脱落状态 `[1]` `R` 复位 = `0`
    ///
    /// 有关 IN2N 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in2n_off, set_in2n_off: 1;
    /// 通道 1 负通道导联脱落状态 `[0]` `R` 复位 = `0`
    ///
    /// 有关 IN1N 电极是打开还是关闭的状态
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub in1n_off, set_in1n_off: 0;
}

bitfield! {
    /// 通用 I/O 寄存器 地址 = `14h` 复位 = `0Fh`
    #[derive(Clone, Copy)]
    pub struct GpioReg(u8);
    impl Debug;
    bool; u8;
    /// GPIO 数据 `[7:4]` `R/W` 复位 = `0`
    ///
    /// 这些位用于从 GPIO 端口读取数据以及向其中写入数据。
    /// 在读取寄存器时，返回的数据对应于 GPIO 外部引脚的状态，
    /// 无论它们是编程为输入还是输出都是如此。作为输出时，对 GPIOD
    /// 进行写入可设置输出值。作为输入时，对 GPIOD 进行写入无效。
    ///
    /// GPIO 在某些呼吸模式下不可用。
    ///
    /// - 0 = 电极打开
    /// - 1 = 电极关闭
    pub gpio_d, set_gpio_d: 7, 4;
    /// GPIO 控制（相应的 GPIOD） `[3:0]` `R/W` 复位 = `1111b = 16`
    ///
    /// 这些位决定相应的 GPIOD 引脚是输入还是输出。
    ///
    /// - 0 = 输出
    /// - 1 = 输入
    pub gpio_c, set_gpio_c: 3, 0;
}

bitfield! {
    /// 起搏信号检测寄存器 地址 = `15h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct PaceReg(u8);
    impl Debug;
    bool; u8;
    /// 保留 `[7:5]` `R/W` 复位 = `0`
    ///
    /// 始终写入 `0`
    pub rev_7_5, set_rev_7_5: 7, 5;
    /// 起搏信号偶数通道 `[4:3]` `R/W` 复位 = `0`
    ///
    /// 这些位控制 TEST_PACE_OUT1 上可用的偶数通道选择。任何时候都只能选择一个通道。
    ///
    /// - 00 = 通道 2
    /// - 01 = 通道 4
    /// - 10 = 通道 6（ADS1296、ADS1296R、ADS1298、ADS1298R）
    /// - 11 = 通道 8（ADS1298 和 ADS1298R）
    pub pace_e, set_pace_e: 4, 3;
    /// 起搏信号奇数通道 `[2:1]` `R/W` 复位 = `0`
    ///
    /// 这些位控制 TEST_PACE_OUT2 上可用的奇数通道选择。任何时候都只能选择一个通道。
    ///
    /// - 00 = 通道 1
    /// - 01 = 通道 3
    /// - 10 = 通道 5（ADS1296、ADS1296R、ADS1298、ADS1298R）
    /// - 11 = 通道 7（ADS1298、ADS1298R）
    pub pace_o, set_pace_o: 2, 1;
    /// 起搏信号检测寄存器 `[0]` `R/W` 复位 = `0`
    ///
    /// 该位用于启用/禁用起搏信号检测缓冲器。
    ///
    /// - 0 = 起搏信号检测缓冲器关闭
    /// - 1 = 起搏信号检测缓冲器开启
    pub pd_pace, set_pd_pace: 0;
}

bitfield! {
    /// 呼吸控制寄存器 地址 = `16h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct RespReg(u8);
    impl Debug;
    bool; u8;
    /// 启用呼吸解调电路（仅限 ADS129xR；对于 ADS129x，始终写入 `0`） `[7]` `R/W` 复位 = `0`
    ///
    /// 该位启用和禁用通道 1 上的解调电路。
    ///
    /// - 0 = RESP 解调电路关闭
    /// - 1 = RESP 解调电路开启
    pub resp_demod_en1, set_resp_demod_en1: 7;
    /// 启用呼吸调制电路（仅限 ADS129xR；对于 ADS129x，始终写入 `0`） `[6]` `R/W` 复位 = `0`
    ///
    /// 该位启用和禁用通道 1 上的调制电路。
    ///
    /// - 0 = RESP 调制电路关闭
    /// - 1 = RESP 调制电路开启
    pub resp_mod_en1, set_resp_mod_en1: 6;
    /// 保留 `[5]` `R/W` 复位 = `0`
    ///
    /// 始终写入 `1`
    pub rev_5, set_rev_5: 5;
    /// 呼吸相位 `[4:2]` `R/W` 复位 = `0`
    ///
    /// - 000 = 22.5°
    /// - 001 = 45°
    /// - 010 = 67.5°
    /// - 011 = 90°
    /// - 100 = 112.5°
    /// - 101 = 135°
    /// - 110 = 157.5°
    /// - 111 = 不适用
    pub resp_phase, set_resp_phase: 4, 2;
    /// 呼吸控制 `[1:0]` `R/W` 复位 = `0`
    ///
    /// 这些位设置呼吸电路的模式。
    ///
    /// - 00 = 无呼吸
    /// - 01 = 外部呼吸
    /// - 10 = 具有内部信号的内部呼吸
    /// - 11 = 具有用户生成的信号的内部呼吸
    pub resp_ctrl, set_resp_ctrl: 1, 0;
}

bitfield! {
    /// 配置寄存器 4 地址 = `17h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct Config4Reg(u8);
    impl Debug;
    bool; u8;
    /// 呼吸调制频率 `[7:5]` `R/W` 复位 `0`
    ///
    /// 当 RESP_CTRL[1:0] = 10 或 RESP_CTRL[1:0] = 10 时，这些位控制呼吸控制频率。
    ///
    /// - 000 = 64kHz 调制时钟
    /// - 001 = 32kHz 调制时钟
    /// - 010 = GPIO3 和 GPIO4 上的 16kHz 方波。GPIO4 上的输出与 GPIO3 的相位相差 180 度。
    /// - 011 = GPIO3 和 GPIO4 上的 8kHz 方波。GPIO4 上的输出与 GPIO3 的相位相差 180 度。
    /// - 100 = GPIO3 和 GPIO4 上的 4kHz 方波。GPIO4 上的输出与 GPIO3 的相位相差 180 度。
    /// - 101 = GPIO3 和 GPIO4 上的 2kHz 方波。GPIO4 上的输出与 GPIO3 的相位相差 180 度。
    /// - 110 = GPIO3 和 GPIO4 上的 1kHz 方波。GPIO4 上的输出与 GPIO3 的相位相差 180 度。
    /// - 111 = GPIO3 和 GPIO4 上的 500Hz 方波。GPIO4 上的输出与 GPIO3 的相位相差 180 度。
    ///
    /// 模式 000 和 001 是内部和外部呼吸模式下的调制频率。在内部呼吸模式下，
    /// 控制信号出现在 RESP_MODP 和 RESP_MODN 端子上。所有其他位设置如上所述在 GPIO4 和 GPIO3 上生成方波。
    pub resp_freq, set_resp_freq: 7, 5;
    /// 保留 `[4]` `R/W` 复位 `0`
    ///
    /// 始终写入 0h
    pub rev_4, set_rev_4: 4;
    /// 单冲转换 `[3]` `R/W` 复位 `0`
    ///
    /// 该位设置转换模式。
    ///
    /// - 0 = 连续转换模式
    /// - 1 = 单冲模式
    pub single_shot, set_single_shot: 3;
    /// 将 WCT 连接到 RLD `[2]` `R/W` 复位 `0`
    ///
    /// 该位将 WCT 连接到 RLD。
    ///
    /// - 0 = WCT 到 RLD 的连接关闭
    /// - 1 = WCT 到 RLD 的连接开启
    pub wct_to_rld, set_wct_to_rld: 2;
    /// 导联脱落比较器断电 `[1]` `R/W` 复位 `0`
    ///
    /// 该位使导联脱落比较器断电。
    ///
    /// - 0 = 禁用导联脱落比较器
    /// - 1 = 启用导联脱落比较器
    pub pd_loff_comp, set_pd_loff_comp: 1;
    /// 保留 `[0]` `R/W` 复位 `0`
    ///
    /// 始终写入 `0`
    pub rev_0, set_rev_0: 0;
}

bitfield! {
    /// 威尔逊中心端子和增强导联控制寄存器 地址 = `18h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct Wct1Reg(u8);
    impl Debug;
    bool; u8;
    /// 使 (WCTA + WCTB)/2 成为通道 6（ADS1296、ADS1296R、ADS1298 和 ADS1298R）的负输入 `[7]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub a_vf_ch6, set_a_vf_ch6: 7;
    /// 使 (WCTA + WCTB)/2 成为通道 5（ADS1296、ADS1296R、ADS1298 和 ADS1298R）的负输入 `[6]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub a_vl_ch5, set_a_vl_ch5: 6;
    /// 使 (WCTB + WCTC)/2 成为通道 7（ADS1298 和 ADS1298R）的负输入 `[5]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub a_vr_ch7, set_a_vr_ch7: 5;
    /// 使 (WCTB + WCTC)/2 成为通道 4 的负输入 `[4]` `R/W` 复位 = `0`
    ///
    /// - 0 = 禁用
    /// - 1 = 启用
    pub avr_ch4, set_avr_ch4: 4;
    /// 使 WCTA 断电 `[3]` `R/W` 复位 = `0`
    ///
    /// - 0 = 断电
    /// - 1 = 上电
    pub pd_wtca, set_pd_wtca: 3;
    /// WCT 放大器 A 通道选择，通常连接到 RA 电极 `[2:0]` `R/W` 复位 = `0`
    ///
    /// 这些位选择通道 1 至 4 的八个电极输入之一。
    ///
    /// - 000 = 通道 1 正输入连接到 WCTA 放大器
    /// - 001 = 通道 1 负输入连接到 WCTA 放大器
    /// - 010 = 通道 2 正输入连接到 WCTA 放大器
    /// - 011 = 通道 2 负输入连接到 WCTA 放大器
    /// - 100 = 通道 3 正输入连接到 WCTA 放大器
    /// - 101 = 通道 3 负输入连接到 WCTA 放大器
    /// - 110 = 通道 4 正输入连接到 WCTA 放大器
    /// - 111 = 通道 4 负输入连接到 WCTA 放大器
    pub wcta_channel, set_wcta_channel: 2, 0;
}

bitfield! {
    /// 威尔逊中心端子控制寄存器 地址 = `19h` 复位 = `00h`
    #[derive(Clone, Copy)]
    pub struct Wct2Reg(u8);
    impl Debug;
    bool; u8;
    /// 使 WCTC 断电 `[7]` `R/W` 复位 = `0`
    ///
    /// - 0 = 断电
    /// - 1 = 上电
    pub pd_wctc, set_pd_wctc: 7;
    /// 使 WCTB 断电 `[6]` `R/W` 复位 = `0`
    ///
    /// - 0 = 断电
    /// - 1 = 上电
    pub pd_wctb, set_pd_wctb: 6;
    /// WCT 放大器 B 通道选择，通常连接到 LA 电极 `[5:3]` `R/W` 复位 = `0`
    ///
    /// 这些位选择通道 1 至 4 的八个电极输入之一。
    ///
    /// - 000 = 通道 1 正输入连接到 WCTB 放大器
    /// - 001 = 通道 1 负输入连接到 WCTB 放大器
    /// - 010 = 通道 2 正输入连接到 WCTB 放大器
    /// - 011 = 通道 2 负输入连接到 WCTB 放大器
    /// - 100 = 通道 3 正输入连接到 WCTB 放大器
    /// - 101 = 通道 3 负输入连接到 WCTB 放大器
    /// - 110 = 通道 4 正输入连接到 WCTB 放大器
    /// - 111 = 通道 4 负输入连接到 WCTB 放大器
    pub wctb_channel, set_wctb_channel: 5, 3;
    /// WCT 放大器 C 通道选择，通常连接到 LL 电极 `[2:0]` `R/W` 复位 = `0`
    ///
    /// 这些位选择通道 1 至 4 的八个电极输入之一。
    ///
    /// - 000 = 通道 1 正输入连接到 WCTC 放大器
    /// - 001 = 通道 1 负输入连接到 WCTC 放大器
    /// - 010 = 通道 2 正输入连接到 WCTC 放大器
    /// - 011 = 通道 2 负输入连接到 WCTC 放大器
    /// - 100 = 通道 3 正输入连接到 WCTC 放大器
    /// - 101 = 通道 3 负输入连接到 WCTC 放大器
    /// - 110 = 通道 4 正输入连接到 WCTC 放大器
    /// - 111 = 通道 4 负输入连接到 WCTC 放大器
    pub wctc_channel, set_wctc_channel: 2, 0;
}
