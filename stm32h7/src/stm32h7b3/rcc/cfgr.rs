#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck and traceclk). Set by hardware in order to: force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
pub type SW_R = crate::FieldReader<u8, SW_A>;
#[doc = "system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck and traceclk). Set by hardware in order to: force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "0: HSI selected as system clock"]
    Hsi = 0,
    #[doc = "1: CSI selected as system clock"]
    Csi = 1,
    #[doc = "2: HSE selected as system clock"]
    Hse = 2,
    #[doc = "3: PLL1 selected as system clock"]
    Pll1 = 3,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SW_A> {
        match self.bits {
            0 => Some(SW_A::Hsi),
            1 => Some(SW_A::Csi),
            2 => Some(SW_A::Hse),
            3 => Some(SW_A::Pll1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Csi`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SW_A::Csi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll1`"]
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SW_A::Pll1
    }
}
#[doc = "Field `SW` writer - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck and traceclk). Set by hardware in order to: force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, SW_A, 3, O>;
impl<'a, const O: u8> SW_W<'a, O> {
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SW_A::Hsi)
    }
    #[doc = "CSI selected as system clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(SW_A::Csi)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::Hse)
    }
    #[doc = "PLL1 selected as system clock"]
    #[inline(always)]
    pub fn pll1(self) -> &'a mut W {
        self.variant(SW_A::Pll1)
    }
}
#[doc = "Field `SWS` reader - system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. others: reserved"]
pub type SWS_R = crate::FieldReader<u8, SWSR_A>;
#[doc = "system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR_A {
    #[doc = "0: HSI oscillator used as system clock"]
    Hsi = 0,
    #[doc = "1: CSI oscillator used as system clock"]
    Csi = 1,
    #[doc = "2: HSE oscillator used as system clock"]
    Hse = 2,
    #[doc = "3: PLL1 used as system clock"]
    Pll1 = 3,
}
impl From<SWSR_A> for u8 {
    #[inline(always)]
    fn from(variant: SWSR_A) -> Self {
        variant as _
    }
}
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWSR_A> {
        match self.bits {
            0 => Some(SWSR_A::Hsi),
            1 => Some(SWSR_A::Csi),
            2 => Some(SWSR_A::Hse),
            3 => Some(SWSR_A::Pll1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Csi`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SWSR_A::Csi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll1`"]
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SWSR_A::Pll1
    }
}
#[doc = "Field `STOPWUCK` reader - system clock selection after a wake up from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. See for details. STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWSÂ =Â 10) or a switch on HSE is requested (SWÂ =10)."]
pub type STOPWUCK_R = crate::BitReader<STOPWUCK_A>;
#[doc = "system clock selection after a wake up from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. See for details. STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWSÂ =Â 10) or a switch on HSE is requested (SWÂ =10).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK_A {
    #[doc = "0: HSI selected as wake up clock from system Stop"]
    Hsi = 0,
    #[doc = "1: CSI selected as wake up clock from system Stop"]
    Csi = 1,
}
impl From<STOPWUCK_A> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPWUCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPWUCK_A {
        match self.bits {
            false => STOPWUCK_A::Hsi,
            true => STOPWUCK_A::Csi,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == STOPWUCK_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Csi`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == STOPWUCK_A::Csi
    }
}
#[doc = "Field `STOPWUCK` writer - system clock selection after a wake up from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. See for details. STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWSÂ =Â 10) or a switch on HSE is requested (SWÂ =10)."]
pub type STOPWUCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, STOPWUCK_A, O>;
impl<'a, const O: u8> STOPWUCK_W<'a, O> {
    #[doc = "HSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::Hsi)
    }
    #[doc = "CSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::Csi)
    }
}
#[doc = "Field `STOPKERWUCK` reader - kernel clock selection after a wake up from system Stop Set and reset by software to select the kernel wakeup clock from system Stop. See for details."]
pub use STOPWUCK_R as STOPKERWUCK_R;
#[doc = "Field `STOPKERWUCK` writer - kernel clock selection after a wake up from system Stop Set and reset by software to select the kernel wakeup clock from system Stop. See for details."]
pub use STOPWUCK_W as STOPKERWUCK_W;
#[doc = "Field `RTCPRE` reader - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1Â MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
pub type RTCPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCPRE` writer - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1Â MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
pub type RTCPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 6, O>;
#[doc = "Field `TIMPRE` reader - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains. Refer to for more details."]
pub type TIMPRE_R = crate::BitReader<TIMPRE_A>;
#[doc = "timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains. Refer to for more details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE_A {
    #[doc = "0: Timer kernel clock equal to 2x pclk by default"]
    DefaultX2 = 0,
    #[doc = "1: Timer kernel clock equal to 4x pclk by default"]
    DefaultX4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::DefaultX2,
            true => TIMPRE_A::DefaultX4,
        }
    }
    #[doc = "Checks if the value of the field is `DefaultX2`"]
    #[inline(always)]
    pub fn is_default_x2(&self) -> bool {
        *self == TIMPRE_A::DefaultX2
    }
    #[doc = "Checks if the value of the field is `DefaultX4`"]
    #[inline(always)]
    pub fn is_default_x4(&self) -> bool {
        *self == TIMPRE_A::DefaultX4
    }
}
#[doc = "Field `TIMPRE` writer - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains. Refer to for more details."]
pub type TIMPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, TIMPRE_A, O>;
impl<'a, const O: u8> TIMPRE_W<'a, O> {
    #[doc = "Timer kernel clock equal to 2x pclk by default"]
    #[inline(always)]
    pub fn default_x2(self) -> &'a mut W {
        self.variant(TIMPRE_A::DefaultX2)
    }
    #[doc = "Timer kernel clock equal to 4x pclk by default"]
    #[inline(always)]
    pub fn default_x4(self) -> &'a mut W {
        self.variant(TIMPRE_A::DefaultX4)
    }
}
#[doc = "Field `MCO1PRE` reader - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO1PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO1PRE` writer - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO1PRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MCO1` reader - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO1_R = crate::FieldReader<u8, MCO1_A>;
#[doc = "Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1_A {
    #[doc = "0: HSI selected for micro-controller clock output"]
    Hsi = 0,
    #[doc = "1: LSE selected for micro-controller clock output"]
    Lse = 1,
    #[doc = "2: HSE selected for micro-controller clock output"]
    Hse = 2,
    #[doc = "3: pll1_q selected for micro-controller clock output"]
    Pll1Q = 3,
    #[doc = "4: HSI48 selected for micro-controller clock output"]
    Hsi48 = 4,
}
impl From<MCO1_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO1_A) -> Self {
        variant as _
    }
}
impl MCO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCO1_A> {
        match self.bits {
            0 => Some(MCO1_A::Hsi),
            1 => Some(MCO1_A::Lse),
            2 => Some(MCO1_A::Hse),
            3 => Some(MCO1_A::Pll1Q),
            4 => Some(MCO1_A::Hsi48),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO1_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO1_A::Lse
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO1_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll1Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == MCO1_A::Pll1Q
    }
    #[doc = "Checks if the value of the field is `Hsi48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCO1_A::Hsi48
    }
}
#[doc = "Field `MCO1` writer - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCO1_A, 3, O>;
impl<'a, const O: u8> MCO1_W<'a, O> {
    #[doc = "HSI selected for micro-controller clock output"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO1_A::Hsi)
    }
    #[doc = "LSE selected for micro-controller clock output"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCO1_A::Lse)
    }
    #[doc = "HSE selected for micro-controller clock output"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO1_A::Hse)
    }
    #[doc = "pll1_q selected for micro-controller clock output"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(MCO1_A::Pll1Q)
    }
    #[doc = "HSI48 selected for micro-controller clock output"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(MCO1_A::Hsi48)
    }
}
#[doc = "Field `MCO2PRE` reader - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO2PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO2PRE` writer - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
pub type MCO2PRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MCO2` reader - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO2_R = crate::FieldReader<u8, MCO2_A>;
#[doc = "microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2_A {
    #[doc = "0: System clock selected for micro-controller clock output"]
    Sysclk = 0,
    #[doc = "1: pll2_p selected for micro-controller clock output"]
    Pll2P = 1,
    #[doc = "2: HSE selected for micro-controller clock output"]
    Hse = 2,
    #[doc = "3: pll1_p selected for micro-controller clock output"]
    Pll1P = 3,
    #[doc = "4: CSI selected for micro-controller clock output"]
    Csi = 4,
    #[doc = "5: LSI selected for micro-controller clock output"]
    Lsi = 5,
}
impl From<MCO2_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO2_A) -> Self {
        variant as _
    }
}
impl MCO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCO2_A> {
        match self.bits {
            0 => Some(MCO2_A::Sysclk),
            1 => Some(MCO2_A::Pll2P),
            2 => Some(MCO2_A::Hse),
            3 => Some(MCO2_A::Pll1P),
            4 => Some(MCO2_A::Csi),
            5 => Some(MCO2_A::Lsi),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Pll2P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == MCO2_A::Pll2P
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll1P`"]
    #[inline(always)]
    pub fn is_pll1_p(&self) -> bool {
        *self == MCO2_A::Pll1P
    }
    #[doc = "Checks if the value of the field is `Csi`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == MCO2_A::Csi
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO2_A::Lsi
    }
}
#[doc = "Field `MCO2` writer - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
pub type MCO2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCO2_A, 3, O>;
impl<'a, const O: u8> MCO2_W<'a, O> {
    #[doc = "System clock selected for micro-controller clock output"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO2_A::Sysclk)
    }
    #[doc = "pll2_p selected for micro-controller clock output"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(MCO2_A::Pll2P)
    }
    #[doc = "HSE selected for micro-controller clock output"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO2_A::Hse)
    }
    #[doc = "pll1_p selected for micro-controller clock output"]
    #[inline(always)]
    pub fn pll1_p(self) -> &'a mut W {
        self.variant(MCO2_A::Pll1P)
    }
    #[doc = "CSI selected for micro-controller clock output"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(MCO2_A::Csi)
    }
    #[doc = "LSI selected for micro-controller clock output"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCO2_A::Lsi)
    }
}
impl R {
    #[doc = "Bits 0:2 - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck and traceclk). Set by hardware in order to: force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. others: reserved"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - system clock selection after a wake up from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. See for details. STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWSÂ =Â 10) or a switch on HSE is requested (SWÂ =10)."]
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - kernel clock selection after a wake up from system Stop Set and reset by software to select the kernel wakeup clock from system Stop. See for details."]
    #[inline(always)]
    pub fn stopkerwuck(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1Â MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains. Refer to for more details."]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:21 - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:24 - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:28 - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck and traceclk). Set by hardware in order to: force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bit 6 - system clock selection after a wake up from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. See for details. STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWSÂ =Â 10) or a switch on HSE is requested (SWÂ =10)."]
    #[inline(always)]
    #[must_use]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<6> {
        STOPWUCK_W::new(self)
    }
    #[doc = "Bit 7 - kernel clock selection after a wake up from system Stop Set and reset by software to select the kernel wakeup clock from system Stop. See for details."]
    #[inline(always)]
    #[must_use]
    pub fn stopkerwuck(&mut self) -> STOPKERWUCK_W<7> {
        STOPKERWUCK_W::new(self)
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1Â MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<8> {
        RTCPRE_W::new(self)
    }
    #[doc = "Bit 15 - timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains. Refer to for more details."]
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<15> {
        TIMPRE_W::new(self)
    }
    #[doc = "Bits 18:21 - MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    #[must_use]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<18> {
        MCO1PRE_W::new(self)
    }
    #[doc = "Bits 22:24 - Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mco1(&mut self) -> MCO1_W<22> {
        MCO1_W::new(self)
    }
    #[doc = "Bits 25:28 - MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
    #[inline(always)]
    #[must_use]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<25> {
        MCO2PRE_W::new(self)
    }
    #[doc = "Bits 29:31 - microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mco2(&mut self) -> MCO2_W<29> {
        MCO2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
