#![allow(non_camel_case_types)]
pub mod addressable;
pub mod data;

use addressable::{Address, Addressable};
use enum_variant_type::EnumVariantType;
use ux::u24;

/// 9.4.1.3 数据检索
#[derive(Clone, Copy, Debug)]
pub enum DataRegister {
    DATA_STATUS(data::DataStatus),
    DATA_CH1(u24),
    DATA_CH2(u24),
    DATA_CH3(u24),
    DATA_CH4(u24),
    DATA_CH5(u24),
    DATA_CH6(u24),
    DATA_CH7(u24),
    DATA_CH8(u24),
}

/// 9.6.1 寄存器说明
#[derive(EnumVariantType)]
pub enum Register {
    /// ID 控制寄存器 地址 = `00h` 复位 = `xxh`
    ID,
    /// 配置寄存器 1 地址 = `01h` 复位 = `06h`
    CONFIG1,
    /// 配置寄存器 2 地址 = `02h` 复位 = `40h`
    CONFIG2,
    /// 配置寄存器 3 地址 = `03h` 复位 = `40h`
    CONFIG3,
    /// 导联脱落控制寄存器 地址 = `04h` 复位 = `00h`
    LOFF,
    /// 通道 1 设置 地址 = `05h` 复位 = `00h`
    CH1SET,
    /// 通道 2 设置 地址 = `06h` 复位 = `00h`
    CH2SET,
    /// 通道 3 设置 地址 = `07h` 复位 = `00h`
    CH3SET,
    /// 通道 4 设置 地址 = `08h` 复位 = `00h`
    CH4SET,
    /// 通道 5 设置 地址 = `09h` 复位 = `00h`
    CH5SET,
    /// 通道 6 设置 地址 = `0Ah` 复位 = `00h`
    CH6SET,
    /// 通道 7 设置 地址 = `0Bh` 复位 = `00h`
    CH7SET,
    /// 通道 8 设置 地址 = `0Ch` 复位 = `00h`
    CH8SET,
    /// RLD 正信号导出寄存器 地址 = `0Dh` 复位 = `00h`
    RLD_SENSP,
    /// RLD 负信号导出寄存器 地址 = `0Eh` 复位 = `00h`
    RLD_SENSN,
    /// 正信号导联脱落检测寄存器 地址 = `0Fh` 复位 = `00h`
    LOFF_SENSP,
    /// 负信号导联脱落检测寄存器 地址 = `10h` 复位 = `00h`
    LOFF_SENSN,
    /// 导联脱落翻转寄存器 地址 = `11h` 复位 = `00h`
    LOFF_FLIP,
    /// 导联脱落正信号状态寄存器 地址 = `12h` 复位 = `00h`
    LOFF_STATP,
    /// 导联脱落负信号状态寄存器 地址 = `13h` 复位 = `00h`
    LOFF_STATN,
    /// 通用 I/O 寄存器 地址 = `14h` 复位 = `0Fh`
    GPIO,
    /// 起搏信号检测寄存器 地址 = `15h` 复位 = `00h`
    PACE,
    /// 呼吸控制寄存器 地址 = `16h` 复位 = `00h`
    RESP,
    /// 配置寄存器 4 地址 = `17h` 复位 = `00h`
    CONFIG4,
    /// 威尔逊中心端子和增强导联控制寄存器 地址 = `18h` 复位 = `00h`
    WCT1,
    /// 威尔逊中心端子控制寄存器 地址 = `19h` 复位 = `00h`
    WCT2,
}

macro_rules! implement_addressable {
    ($struct: ty, $value: expr) => {
        impl Addressable for $struct {
            fn get_address(&self) -> Address {
                return $value;
            }
        }
    };
}

implement_addressable!(ID, 0x00);
implement_addressable!(CONFIG1, 0x01);
implement_addressable!(CONFIG2, 0x02);
implement_addressable!(CONFIG3, 0x03);
implement_addressable!(LOFF, 0x04);
implement_addressable!(CH1SET, 0x05);
implement_addressable!(CH2SET, 0x06);
implement_addressable!(CH3SET, 0x07);
implement_addressable!(CH4SET, 0x08);
implement_addressable!(CH5SET, 0x09);
implement_addressable!(CH6SET, 0x0a);
implement_addressable!(CH7SET, 0x0b);
implement_addressable!(CH8SET, 0x0c);
implement_addressable!(RLD_SENSP, 0x0d);
implement_addressable!(RLD_SENSN, 0x0e);
implement_addressable!(LOFF_SENSP, 0x0f);
implement_addressable!(LOFF_SENSN, 0x10);
implement_addressable!(LOFF_FLIP, 0x11);
implement_addressable!(LOFF_STATP, 0x12);
implement_addressable!(LOFF_STATN, 0x13);
implement_addressable!(GPIO, 0x14);
implement_addressable!(PACE, 0x15);
implement_addressable!(RESP, 0x16);
implement_addressable!(CONFIG4, 0x17);
implement_addressable!(WCT1, 0x18);
implement_addressable!(WCT2, 0x19);
