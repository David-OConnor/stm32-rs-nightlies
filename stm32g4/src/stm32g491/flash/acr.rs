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
    #[doc = "0: Zero Wait States (Vcore Boost 1 (<= 34MHz), Vcore Normal 1 (<= 30MHz), Vcore 2 (<= 12MHz)"]
    Wait0 = 0,
    #[doc = "1: One Wait State (Vcore Boost 1 (<= 68MHz), Vcore Normal 1 (<= 60MHz), Vcore 2 (<= 24MHz)"]
    Wait1 = 1,
    #[doc = "2: Two Wait States (Vcore Boost 1 (<= 102MHz), Vcore Normal 1 (<= 90MHz), Vcore 2 (<= 26MHz)"]
    Wait2 = 2,
    #[doc = "3: Three Wait States (Vcore Boost 1 (<= 136MHz), Vcore Normal 1 (<= 120MHz)"]
    Wait3 = 3,
    #[doc = "4: Four Wait States (Vcore Boost 1 (<= 170MHz), Vcore Normal 1 (<= 150MHz)"]
    Wait4 = 4,
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
            0 => Some(LATENCY_A::Wait0),
            1 => Some(LATENCY_A::Wait1),
            2 => Some(LATENCY_A::Wait2),
            3 => Some(LATENCY_A::Wait3),
            4 => Some(LATENCY_A::Wait4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Wait0`"]
    #[inline(always)]
    pub fn is_wait0(&self) -> bool {
        *self == LATENCY_A::Wait0
    }
    #[doc = "Checks if the value of the field is `Wait1`"]
    #[inline(always)]
    pub fn is_wait1(&self) -> bool {
        *self == LATENCY_A::Wait1
    }
    #[doc = "Checks if the value of the field is `Wait2`"]
    #[inline(always)]
    pub fn is_wait2(&self) -> bool {
        *self == LATENCY_A::Wait2
    }
    #[doc = "Checks if the value of the field is `Wait3`"]
    #[inline(always)]
    pub fn is_wait3(&self) -> bool {
        *self == LATENCY_A::Wait3
    }
    #[doc = "Checks if the value of the field is `Wait4`"]
    #[inline(always)]
    pub fn is_wait4(&self) -> bool {
        *self == LATENCY_A::Wait4
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR_SPEC, u8, LATENCY_A, 4, O>;
impl<'a, const O: u8> LATENCY_W<'a, O> {
    #[doc = "Zero Wait States (Vcore Boost 1 (<= 34MHz), Vcore Normal 1 (<= 30MHz), Vcore 2 (<= 12MHz)"]
    #[inline(always)]
    pub fn wait0(self) -> &'a mut W {
        self.variant(LATENCY_A::Wait0)
    }
    #[doc = "One Wait State (Vcore Boost 1 (<= 68MHz), Vcore Normal 1 (<= 60MHz), Vcore 2 (<= 24MHz)"]
    #[inline(always)]
    pub fn wait1(self) -> &'a mut W {
        self.variant(LATENCY_A::Wait1)
    }
    #[doc = "Two Wait States (Vcore Boost 1 (<= 102MHz), Vcore Normal 1 (<= 90MHz), Vcore 2 (<= 26MHz)"]
    #[inline(always)]
    pub fn wait2(self) -> &'a mut W {
        self.variant(LATENCY_A::Wait2)
    }
    #[doc = "Three Wait States (Vcore Boost 1 (<= 136MHz), Vcore Normal 1 (<= 120MHz)"]
    #[inline(always)]
    pub fn wait3(self) -> &'a mut W {
        self.variant(LATENCY_A::Wait3)
    }
    #[doc = "Four Wait States (Vcore Boost 1 (<= 170MHz), Vcore Normal 1 (<= 150MHz)"]
    #[inline(always)]
    pub fn wait4(self) -> &'a mut W {
        self.variant(LATENCY_A::Wait4)
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<bool>;
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub type ICEN_R = crate::BitReader<bool>;
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub type ICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `DCEN` reader - Data cache enable"]
pub type DCEN_R = crate::BitReader<bool>;
#[doc = "Field `DCEN` writer - Data cache enable"]
pub type DCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `ICRST` reader - Instruction cache reset"]
pub type ICRST_R = crate::BitReader<bool>;
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub type ICRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `DCRST` reader - Data cache reset"]
pub type DCRST_R = crate::BitReader<bool>;
#[doc = "Field `DCRST` writer - Data cache reset"]
pub type DCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `RUN_PD` reader - Flash Power-down mode during Low-power run mode"]
pub type RUN_PD_R = crate::BitReader<bool>;
#[doc = "Field `RUN_PD` writer - Flash Power-down mode during Low-power run mode"]
pub type RUN_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `SLEEP_PD` reader - Flash Power-down mode during Low-power sleep mode"]
pub type SLEEP_PD_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_PD` writer - Flash Power-down mode during Low-power sleep mode"]
pub type SLEEP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `DBG_SWEN` reader - Debug software enable"]
pub type DBG_SWEN_R = crate::BitReader<bool>;
#[doc = "Field `DBG_SWEN` writer - Debug software enable"]
pub type DBG_SWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
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
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Debug software enable"]
    #[inline(always)]
    pub fn dbg_swen(&self) -> DBG_SWEN_R {
        DBG_SWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Latency"]
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
    #[doc = "Bit 13 - Flash Power-down mode during Low-power run mode"]
    #[inline(always)]
    #[must_use]
    pub fn run_pd(&mut self) -> RUN_PD_W<13> {
        RUN_PD_W::new(self)
    }
    #[doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<14> {
        SLEEP_PD_W::new(self)
    }
    #[doc = "Bit 18 - Debug software enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_swen(&mut self) -> DBG_SWEN_W<18> {
        DBG_SWEN_W::new(self)
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
