#[doc = "Register `SBS_ECCNMIR` reader"]
pub struct R(crate::R<SBS_ECCNMIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_ECCNMIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_ECCNMIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_ECCNMIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_ECCNMIR` writer"]
pub struct W(crate::W<SBS_ECCNMIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_ECCNMIR_SPEC>;
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
impl From<crate::W<SBS_ECCNMIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_ECCNMIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCNMI_MASK_EN` reader - NMI behavior setup when a double ECC error occurs on flitf data part"]
pub type ECCNMI_MASK_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECCNMI_MASK_EN` writer - NMI behavior setup when a double ECC error occurs on flitf data part"]
pub type ECCNMI_MASK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_ECCNMIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - NMI behavior setup when a double ECC error occurs on flitf data part"]
    #[inline(always)]
    pub fn eccnmi_mask_en(&self) -> ECCNMI_MASK_EN_R {
        ECCNMI_MASK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMI behavior setup when a double ECC error occurs on flitf data part"]
    #[inline(always)]
    #[must_use]
    pub fn eccnmi_mask_en(&mut self) -> ECCNMI_MASK_EN_W<0> {
        ECCNMI_MASK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS flift ECC NMI mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_eccnmir](index.html) module"]
pub struct SBS_ECCNMIR_SPEC;
impl crate::RegisterSpec for SBS_ECCNMIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_eccnmir::R](R) reader structure"]
impl crate::Readable for SBS_ECCNMIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_eccnmir::W](W) writer structure"]
impl crate::Writable for SBS_ECCNMIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_ECCNMIR to value 0"]
impl crate::Resettable for SBS_ECCNMIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
