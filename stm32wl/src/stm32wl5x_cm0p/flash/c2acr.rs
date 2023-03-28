#[doc = "Register `C2ACR` reader"]
pub struct R(crate::R<C2ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2ACR` writer"]
pub struct W(crate::W<C2ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2ACR_SPEC>;
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
impl From<crate::W<C2ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRFTEN` reader - CPU2 Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<PRFTEN_A>;
#[doc = "CPU2 Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN_A {
    #[doc = "0: CPU2 prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: CPU2 prefetch is enabled"]
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
#[doc = "Field `PRFTEN` writer - CPU2 Prefetch enable"]
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2ACR_SPEC, PRFTEN_A, O>;
impl<'a, const O: u8> PRFTEN_W<'a, O> {
    #[doc = "CPU2 prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Disabled)
    }
    #[doc = "CPU2 prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Enabled)
    }
}
#[doc = "Field `ICEN` reader - CPU2 Instruction cache enable"]
pub type ICEN_R = crate::BitReader<ICEN_A>;
#[doc = "CPU2 Instruction cache enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN_A {
    #[doc = "0: CPU2 instruction cache is disabled"]
    Disabled = 0,
    #[doc = "1: CPU2 instruction cache is enabled"]
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
#[doc = "Field `ICEN` writer - CPU2 Instruction cache enable"]
pub type ICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2ACR_SPEC, ICEN_A, O>;
impl<'a, const O: u8> ICEN_W<'a, O> {
    #[doc = "CPU2 instruction cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ICEN_A::Disabled)
    }
    #[doc = "CPU2 instruction cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ICEN_A::Enabled)
    }
}
#[doc = "Field `ICRST` reader - CPU2 Instruction cache reset"]
pub type ICRST_R = crate::BitReader<ICRST_A>;
#[doc = "CPU2 Instruction cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICRST_A {
    #[doc = "0: CPU2 instruction cache is not reset"]
    NotReset = 0,
    #[doc = "1: CPU2 instruction cache is reset"]
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
#[doc = "Field `ICRST` writer - CPU2 Instruction cache reset"]
pub type ICRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2ACR_SPEC, ICRST_A, O>;
impl<'a, const O: u8> ICRST_W<'a, O> {
    #[doc = "CPU2 instruction cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(ICRST_A::NotReset)
    }
    #[doc = "CPU2 instruction cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ICRST_A::Reset)
    }
}
#[doc = "Field `PES` reader - CPU2 program / erase suspend request"]
pub type PES_R = crate::BitReader<PES_A>;
#[doc = "CPU2 program / erase suspend request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PES_A {
    #[doc = "0: Flash program and erase operations granted"]
    Granted = 0,
    #[doc = "1: Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set"]
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
#[doc = "Field `PES` writer - CPU2 program / erase suspend request"]
pub type PES_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2ACR_SPEC, PES_A, O>;
impl<'a, const O: u8> PES_W<'a, O> {
    #[doc = "Flash program and erase operations granted"]
    #[inline(always)]
    pub fn granted(self) -> &'a mut W {
        self.variant(PES_A::Granted)
    }
    #[doc = "Any new Flash program and erase operation is suspended until this bit is cleared. The PESD bit in FLASH_C2SR is set when PES bit in FLASH_C2ACR is set"]
    #[inline(always)]
    pub fn suspended(self) -> &'a mut W {
        self.variant(PES_A::Suspended)
    }
}
impl R {
    #[doc = "Bit 8 - CPU2 Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU2 Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU2 Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU2 program / erase suspend request"]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - CPU2 Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<8> {
        PRFTEN_W::new(self)
    }
    #[doc = "Bit 9 - CPU2 Instruction cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<9> {
        ICEN_W::new(self)
    }
    #[doc = "Bit 11 - CPU2 Instruction cache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<11> {
        ICRST_W::new(self)
    }
    #[doc = "Bit 15 - CPU2 program / erase suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<15> {
        PES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash CPU2 access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2acr](index.html) module"]
pub struct C2ACR_SPEC;
impl crate::RegisterSpec for C2ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2acr::R](R) reader structure"]
impl crate::Readable for C2ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2acr::W](W) writer structure"]
impl crate::Writable for C2ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2ACR to value 0x0600"]
impl crate::Resettable for C2ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
