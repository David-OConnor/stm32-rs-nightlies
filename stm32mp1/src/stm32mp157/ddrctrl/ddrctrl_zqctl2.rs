#[doc = "Register `DDRCTRL_ZQCTL2` reader"]
pub struct R(crate::R<DDRCTRL_ZQCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ZQCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ZQCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ZQCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ZQCTL2` writer"]
pub struct W(crate::W<DDRCTRL_ZQCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ZQCTL2_SPEC>;
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
impl From<crate::W<DDRCTRL_ZQCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ZQCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZQ_RESET` reader - ZQ_RESET"]
pub type ZQ_RESET_R = crate::BitReader<bool>;
#[doc = "Field `ZQ_RESET` writer - ZQ_RESET"]
pub type ZQ_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_ZQCTL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ZQ_RESET"]
    #[inline(always)]
    pub fn zq_reset(&self) -> ZQ_RESET_R {
        ZQ_RESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ZQ_RESET"]
    #[inline(always)]
    #[must_use]
    pub fn zq_reset(&mut self) -> ZQ_RESET_W<0> {
        ZQ_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL ZQ control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqctl2](index.html) module"]
pub struct DDRCTRL_ZQCTL2_SPEC;
impl crate::RegisterSpec for DDRCTRL_ZQCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_zqctl2::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_zqctl2::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ZQCTL2 to value 0"]
impl crate::Resettable for DDRCTRL_ZQCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
