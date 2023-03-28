#[doc = "Register `EWCR` reader"]
pub struct R(crate::R<EWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EWCR` writer"]
pub struct W(crate::W<EWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EWCR_SPEC>;
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
impl From<crate::W<EWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWIT` reader - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]
- 1. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
pub type EWIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EWIT` writer - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]
- 1. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
pub type EWIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EWCR_SPEC, u16, u16, 12, O>;
#[doc = "Field `EWIC` writer - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0."]
pub type EWIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EWCR_SPEC, bool, O>;
#[doc = "Field `EWIE` reader - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
pub type EWIE_R = crate::BitReader<bool>;
#[doc = "Field `EWIE` writer - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
pub type EWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EWCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]
- 1. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
    #[inline(always)]
    pub fn ewit(&self) -> EWIT_R {
        EWIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\]
- 1. EWIT\\[11:0\\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
    #[inline(always)]
    #[must_use]
    pub fn ewit(&mut self) -> EWIT_W<0> {
        EWIT_W::new(self)
    }
    #[doc = "Bit 14 - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0."]
    #[inline(always)]
    #[must_use]
    pub fn ewic(&mut self) -> EWIC_W<14> {
        EWIC_W::new(self)
    }
    #[doc = "Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EWIE_W<15> {
        EWIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IWDG early wakeup interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ewcr](index.html) module"]
pub struct EWCR_SPEC;
impl crate::RegisterSpec for EWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ewcr::R](R) reader structure"]
impl crate::Readable for EWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ewcr::W](W) writer structure"]
impl crate::Writable for EWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EWCR to value 0"]
impl crate::Resettable for EWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
