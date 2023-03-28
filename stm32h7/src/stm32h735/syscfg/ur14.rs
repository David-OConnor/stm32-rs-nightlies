#[doc = "Register `UR14` reader"]
pub struct R(crate::R<UR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR14` writer"]
pub struct W(crate::W<UR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR14_SPEC>;
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
impl From<crate::W<UR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D1STPRST` reader - D1 Stop Reset"]
pub type D1STPRST_R = crate::BitReader<bool>;
#[doc = "Field `D1STPRST` writer - D1 Stop Reset"]
pub type D1STPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, UR14_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    pub fn d1stprst(&self) -> D1STPRST_R {
        D1STPRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    #[must_use]
    pub fn d1stprst(&mut self) -> D1STPRST_W<0> {
        D1STPRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur14](index.html) module"]
pub struct UR14_SPEC;
impl crate::RegisterSpec for UR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur14::R](R) reader structure"]
impl crate::Readable for UR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur14::W](W) writer structure"]
impl crate::Writable for UR14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR14 to value 0"]
impl crate::Resettable for UR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
