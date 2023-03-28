#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::FieldReader<u8, LATENCY_A>;
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY_A {
    #[doc = "0: 0 wait states"]
    Ws0 = 0,
    #[doc = "1: 1 wait states"]
    Ws1 = 1,
    #[doc = "2: 2 wait states"]
    Ws2 = 2,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LATENCY_A> {
        match self.bits {
            0 => Some(LATENCY_A::Ws0),
            1 => Some(LATENCY_A::Ws1),
            2 => Some(LATENCY_A::Ws2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Ws0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY_A::Ws0
    }
    #[doc = "Checks if the value of the field is `Ws1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY_A::Ws1
    }
    #[doc = "Checks if the value of the field is `Ws2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY_A::Ws2
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR_SPEC, u8, LATENCY_A, 3, O>;
impl<'a, const O: u8> LATENCY_W<'a, O> {
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws0)
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws2)
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<PRFTEN_A>;
#[doc = "Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN_A {
    #[doc = "0: Prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTEN_A {
        match self.bits {
            false => PRFTEN_A::Disabled,
            true => PRFTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN_A::Enabled
    }
}
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, PRFTEN_A, O>;
impl<'a, const O: u8> PRFTEN_W<'a, O> {
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Disabled)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Enabled)
    }
}
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub type ICEN_R = crate::BitReader<ICEN_A>;
#[doc = "Instruction cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN_A {
    #[doc = "0: Instruction cache is disabled"]
    Disabled = 0,
    #[doc = "1: Instruction cache is enabled"]
    Enabled = 1,
}
impl From<ICEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEN_A {
        match self.bits {
            false => ICEN_A::Disabled,
            true => ICEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICEN_A::Enabled
    }
}
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub type ICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, ICEN_A, O>;
impl<'a, const O: u8> ICEN_W<'a, O> {
    #[doc = "Instruction cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ICEN_A::Disabled)
    }
    #[doc = "Instruction cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ICEN_A::Enabled)
    }
}
#[doc = "Field `DCEN` reader - Data cache enable"]
pub type DCEN_R = crate::BitReader<DCEN_A>;
#[doc = "Data cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEN_A {
    #[doc = "0: Data cache is disabled"]
    Disabled = 0,
    #[doc = "1: Data cache is enabled"]
    Enabled = 1,
}
impl From<DCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCEN_A {
        match self.bits {
            false => DCEN_A::Disabled,
            true => DCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCEN_A::Enabled
    }
}
#[doc = "Field `DCEN` writer - Data cache enable"]
pub type DCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, DCEN_A, O>;
impl<'a, const O: u8> DCEN_W<'a, O> {
    #[doc = "Data cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCEN_A::Disabled)
    }
    #[doc = "Data cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCEN_A::Enabled)
    }
}
#[doc = "Field `ICRST` reader - Instruction cache reset"]
pub type ICRST_R = crate::BitReader<ICRST_A>;
#[doc = "Instruction cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRST_A {
    #[doc = "0: Instruction cache is not reset"]
    NotReset = 0,
    #[doc = "1: Instruction cache is reset"]
    Reset = 1,
}
impl From<ICRST_A> for bool {
    #[inline(always)]
    fn from(variant: ICRST_A) -> Self {
        variant as u8 != 0
    }
}
impl ICRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICRST_A {
        match self.bits {
            false => ICRST_A::NotReset,
            true => ICRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NotReset`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == ICRST_A::NotReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ICRST_A::Reset
    }
}
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub type ICRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, ICRST_A, O>;
impl<'a, const O: u8> ICRST_W<'a, O> {
    #[doc = "Instruction cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(ICRST_A::NotReset)
    }
    #[doc = "Instruction cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ICRST_A::Reset)
    }
}
#[doc = "Field `DCRST` reader - Data cache reset"]
pub type DCRST_R = crate::BitReader<DCRST_A>;
#[doc = "Data cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCRST_A {
    #[doc = "0: Data cache is not reset"]
    NotReset = 0,
    #[doc = "1: Data cache is reset"]
    Reset = 1,
}
impl From<DCRST_A> for bool {
    #[inline(always)]
    fn from(variant: DCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl DCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCRST_A {
        match self.bits {
            false => DCRST_A::NotReset,
            true => DCRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NotReset`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == DCRST_A::NotReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCRST_A::Reset
    }
}
#[doc = "Field `DCRST` writer - Data cache reset"]
pub type DCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, DCRST_A, O>;
impl<'a, const O: u8> DCRST_W<'a, O> {
    #[doc = "Data cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(DCRST_A::NotReset)
    }
    #[doc = "Data cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DCRST_A::Reset)
    }
}
#[doc = "Field `PES` reader - CPU1 programm erase suspend request"]
pub type PES_R = crate::BitReader<PES_A>;
#[doc = "CPU1 programm erase suspend request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PES_A {
    #[doc = "0: Flash program and erase operations granted"]
    Granted = 0,
    #[doc = "1: Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_SR is set when PES bit in FLASH_ACR is set"]
    Suspended = 1,
}
impl From<PES_A> for bool {
    #[inline(always)]
    fn from(variant: PES_A) -> Self {
        variant as u8 != 0
    }
}
impl PES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PES_A {
        match self.bits {
            false => PES_A::Granted,
            true => PES_A::Suspended,
        }
    }
    #[doc = "Checks if the value of the field is `Granted`"]
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        *self == PES_A::Granted
    }
    #[doc = "Checks if the value of the field is `Suspended`"]
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == PES_A::Suspended
    }
}
#[doc = "Field `PES` writer - CPU1 programm erase suspend request"]
pub type PES_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, PES_A, O>;
impl<'a, const O: u8> PES_W<'a, O> {
    #[doc = "Flash program and erase operations granted"]
    #[inline(always)]
    pub fn granted(self) -> &'a mut W {
        self.variant(PES_A::Granted)
    }
    #[doc = "Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_SR is set when PES bit in FLASH_ACR is set"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut W {
        self.variant(PES_A::Suspended)
    }
}
#[doc = "Field `EMPTY` reader - Flash User area empty"]
pub type EMPTY_R = crate::BitReader<EMPTY_A>;
#[doc = "Flash User area empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMPTY_A {
    #[doc = "0: User Flash programmend"]
    Programmed = 0,
    #[doc = "1: User Flash empty"]
    Empty = 1,
}
impl From<EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMPTY_A {
        match self.bits {
            false => EMPTY_A::Programmed,
            true => EMPTY_A::Empty,
        }
    }
    #[doc = "Checks if the value of the field is `Programmed`"]
    #[inline(always)]
    pub fn is_programmed(&self) -> bool {
        *self == EMPTY_A::Programmed
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == EMPTY_A::Empty
    }
}
#[doc = "Field `EMPTY` writer - Flash User area empty"]
pub type EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, EMPTY_A, O>;
impl<'a, const O: u8> EMPTY_W<'a, O> {
    #[doc = "User Flash programmend"]
    #[inline(always)]
    pub fn programmed(self) -> &'a mut W {
        self.variant(EMPTY_A::Programmed)
    }
    #[doc = "User Flash empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(EMPTY_A::Empty)
    }
}
impl R {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU1 programm erase suspend request"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Flash User area empty"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<8> {
        PRFTEN_W::new(self)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<9> {
        ICEN_W::new(self)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcen(&mut self) -> DCEN_W<10> {
        DCEN_W::new(self)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<11> {
        ICRST_W::new(self)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcrst(&mut self) -> DCRST_W<12> {
        DCRST_W::new(self)
    }
    #[doc = "Bit 15 - CPU1 programm erase suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<15> {
        PES_W::new(self)
    }
    #[doc = "Bit 16 - Flash User area empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EMPTY_W<16> {
        EMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0x0600"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
