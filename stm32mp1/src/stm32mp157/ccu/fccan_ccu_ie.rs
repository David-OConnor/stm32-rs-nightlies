#[doc = "Register `FCCAN_CCU_IE` reader"]
pub struct R(crate::R<FCCAN_CCU_IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCAN_CCU_IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCAN_CCU_IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCAN_CCU_IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCAN_CCU_IE` writer"]
pub struct W(crate::W<FCCAN_CCU_IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCAN_CCU_IE_SPEC>;
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
impl From<crate::W<FCCAN_CCU_IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCAN_CCU_IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWEE` reader - CWEE"]
pub type CWEE_R = crate::BitReader<bool>;
#[doc = "Field `CWEE` writer - CWEE"]
pub type CWEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCCAN_CCU_IE_SPEC, bool, O>;
#[doc = "Field `CSCE` reader - CSCE"]
pub type CSCE_R = crate::BitReader<bool>;
#[doc = "Field `CSCE` writer - CSCE"]
pub type CSCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCCAN_CCU_IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CWEE"]
    #[inline(always)]
    pub fn cwee(&self) -> CWEE_R {
        CWEE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSCE"]
    #[inline(always)]
    pub fn csce(&self) -> CSCE_R {
        CSCE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CWEE"]
    #[inline(always)]
    #[must_use]
    pub fn cwee(&mut self) -> CWEE_W<0> {
        CWEE_W::new(self)
    }
    #[doc = "Bit 1 - CSCE"]
    #[inline(always)]
    #[must_use]
    pub fn csce(&mut self) -> CSCE_W<1> {
        CSCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The settings in the CU interrupt enable register determine whether a status change in the CU interrupt register will be signaled on an interrupt line.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccan_ccu_ie](index.html) module"]
pub struct FCCAN_CCU_IE_SPEC;
impl crate::RegisterSpec for FCCAN_CCU_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fccan_ccu_ie::R](R) reader structure"]
impl crate::Readable for FCCAN_CCU_IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fccan_ccu_ie::W](W) writer structure"]
impl crate::Writable for FCCAN_CCU_IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCCAN_CCU_IE to value 0"]
impl crate::Resettable for FCCAN_CCU_IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
