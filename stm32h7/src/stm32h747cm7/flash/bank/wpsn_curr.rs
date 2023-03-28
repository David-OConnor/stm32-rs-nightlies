#[doc = "Register `WPSN_CURR` reader"]
pub struct R(crate::R<WPSN_CURR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPSN_CURR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPSN_CURR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPSN_CURR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPSN_CURR` writer"]
pub struct W(crate::W<WPSN_CURR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPSN_CURR_SPEC>;
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
impl From<crate::W<WPSN_CURR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPSN_CURR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRPSn` reader - Bank 1 sector write protection option status byte"]
pub type WRPSN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRPSn` writer - Bank 1 sector write protection option status byte"]
pub type WRPSN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPSN_CURR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    #[must_use]
    pub fn wrpsn(&mut self) -> WRPSN_W<0> {
        WRPSN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH write sector protection for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsn_curr](index.html) module"]
pub struct WPSN_CURR_SPEC;
impl crate::RegisterSpec for WPSN_CURR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpsn_curr::R](R) reader structure"]
impl crate::Readable for WPSN_CURR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpsn_curr::W](W) writer structure"]
impl crate::Writable for WPSN_CURR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPSN_CURR to value 0"]
impl crate::Resettable for WPSN_CURR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
