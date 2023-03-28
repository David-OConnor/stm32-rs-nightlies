#[doc = "Register `OTFDEC_IER` reader"]
pub struct R(crate::R<OTFDEC_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTFDEC_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTFDEC_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTFDEC_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTFDEC_IER` writer"]
pub struct W(crate::W<OTFDEC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTFDEC_IER_SPEC>;
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
impl From<crate::W<OTFDEC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTFDEC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEIE` reader - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set."]
pub type SEIE_R = crate::BitReader<bool>;
#[doc = "Field `SEIE` writer - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set."]
pub type SEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_IER_SPEC, bool, O>;
#[doc = "Field `XONEIE` reader - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set."]
pub type XONEIE_R = crate::BitReader<bool>;
#[doc = "Field `XONEIE` writer - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set."]
pub type XONEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_IER_SPEC, bool, O>;
#[doc = "Field `KEIE` reader - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set."]
pub type KEIE_R = crate::BitReader<bool>;
#[doc = "Field `KEIE` writer - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set."]
pub type KEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTFDEC_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set."]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set."]
    #[inline(always)]
    pub fn xoneie(&self) -> XONEIE_R {
        XONEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set."]
    #[inline(always)]
    pub fn keie(&self) -> KEIE_R {
        KEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set."]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<0> {
        SEIE_W::new(self)
    }
    #[doc = "Bit 1 - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set."]
    #[inline(always)]
    #[must_use]
    pub fn xoneie(&mut self) -> XONEIE_W<1> {
        XONEIE_W::new(self)
    }
    #[doc = "Bit 2 - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set."]
    #[inline(always)]
    #[must_use]
    pub fn keie(&mut self) -> KEIE_W<2> {
        KEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otfdec_ier](index.html) module"]
pub struct OTFDEC_IER_SPEC;
impl crate::RegisterSpec for OTFDEC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otfdec_ier::R](R) reader structure"]
impl crate::Readable for OTFDEC_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otfdec_ier::W](W) writer structure"]
impl crate::Writable for OTFDEC_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTFDEC_IER to value 0"]
impl crate::Resettable for OTFDEC_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
