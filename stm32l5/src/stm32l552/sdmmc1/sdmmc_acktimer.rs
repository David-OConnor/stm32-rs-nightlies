#[doc = "Register `SDMMC_ACKTIMER` reader"]
pub struct R(crate::R<SDMMC_ACKTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_ACKTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_ACKTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_ACKTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_ACKTIMER` writer"]
pub struct W(crate::W<SDMMC_ACKTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_ACKTIMER_SPEC>;
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
impl From<crate::W<SDMMC_ACKTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_ACKTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACKTIME` reader - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
pub type ACKTIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ACKTIME` writer - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
pub type ACKTIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_ACKTIMER_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
    #[inline(always)]
    pub fn acktime(&self) -> ACKTIME_R {
        ACKTIME_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
    #[inline(always)]
    #[must_use]
    pub fn acktime(&mut self) -> ACKTIME_W<0> {
        ACKTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_acktimer](index.html) module"]
pub struct SDMMC_ACKTIMER_SPEC;
impl crate::RegisterSpec for SDMMC_ACKTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_acktimer::R](R) reader structure"]
impl crate::Readable for SDMMC_ACKTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_acktimer::W](W) writer structure"]
impl crate::Writable for SDMMC_ACKTIMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDMMC_ACKTIMER to value 0"]
impl crate::Resettable for SDMMC_ACKTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
