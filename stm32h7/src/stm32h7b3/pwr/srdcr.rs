#[doc = "Register `SRDCR` reader"]
pub struct R(crate::R<SRDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRDCR` writer"]
pub struct W(crate::W<SRDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRDCR_SPEC>;
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
impl From<crate::W<SRDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOSRDY` reader - VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
pub type VOSRDY_R = crate::BitReader<bool>;
#[doc = "Field `VOS` reader - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
pub type VOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOS` writer - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRDCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 13 - VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<14> {
        VOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdcr](index.html) module"]
pub struct SRDCR_SPEC;
impl crate::RegisterSpec for SRDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdcr::R](R) reader structure"]
impl crate::Readable for SRDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srdcr::W](W) writer structure"]
impl crate::Writable for SRDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRDCR to value 0x4000"]
impl crate::Resettable for SRDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
