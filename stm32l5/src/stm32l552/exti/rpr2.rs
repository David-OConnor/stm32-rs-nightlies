#[doc = "Register `RPR2` reader"]
pub struct R(crate::R<RPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPR2` writer"]
pub struct W(crate::W<RPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR2_SPEC>;
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
impl From<crate::W<RPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPIF35` reader - RPIF35"]
pub type RPIF35_R = crate::BitReader<bool>;
#[doc = "Field `RPIF35` writer - RPIF35"]
pub type RPIF35_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPR2_SPEC, bool, O>;
#[doc = "Field `RPIF36` reader - RPIF36"]
pub type RPIF36_R = crate::BitReader<bool>;
#[doc = "Field `RPIF36` writer - RPIF36"]
pub type RPIF36_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPR2_SPEC, bool, O>;
#[doc = "Field `RPIF37` reader - RPIF37"]
pub type RPIF37_R = crate::BitReader<bool>;
#[doc = "Field `RPIF37` writer - RPIF37"]
pub type RPIF37_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPR2_SPEC, bool, O>;
#[doc = "Field `RPIF38` reader - RPIF38"]
pub type RPIF38_R = crate::BitReader<bool>;
#[doc = "Field `RPIF38` writer - RPIF38"]
pub type RPIF38_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - RPIF35"]
    #[inline(always)]
    pub fn rpif35(&self) -> RPIF35_R {
        RPIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RPIF36"]
    #[inline(always)]
    pub fn rpif36(&self) -> RPIF36_R {
        RPIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RPIF37"]
    #[inline(always)]
    pub fn rpif37(&self) -> RPIF37_R {
        RPIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RPIF38"]
    #[inline(always)]
    pub fn rpif38(&self) -> RPIF38_R {
        RPIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - RPIF35"]
    #[inline(always)]
    #[must_use]
    pub fn rpif35(&mut self) -> RPIF35_W<3> {
        RPIF35_W::new(self)
    }
    #[doc = "Bit 4 - RPIF36"]
    #[inline(always)]
    #[must_use]
    pub fn rpif36(&mut self) -> RPIF36_W<4> {
        RPIF36_W::new(self)
    }
    #[doc = "Bit 5 - RPIF37"]
    #[inline(always)]
    #[must_use]
    pub fn rpif37(&mut self) -> RPIF37_W<5> {
        RPIF37_W::new(self)
    }
    #[doc = "Bit 6 - RPIF38"]
    #[inline(always)]
    #[must_use]
    pub fn rpif38(&mut self) -> RPIF38_W<6> {
        RPIF38_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI rising edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr2](index.html) module"]
pub struct RPR2_SPEC;
impl crate::RegisterSpec for RPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpr2::R](R) reader structure"]
impl crate::Readable for RPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpr2::W](W) writer structure"]
impl crate::Writable for RPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPR2 to value 0"]
impl crate::Resettable for RPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
