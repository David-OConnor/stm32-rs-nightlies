#[doc = "Register `SWIER2` reader"]
pub struct R(crate::R<SWIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER2` writer"]
pub struct W(crate::W<SWIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER2_SPEC>;
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
impl From<crate::W<SWIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI35` reader - SWI35"]
pub type SWI35_R = crate::BitReader<bool>;
#[doc = "Field `SWI35` writer - SWI35"]
pub type SWI35_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, bool, O>;
#[doc = "Field `SWI36` reader - SWI36"]
pub type SWI36_R = crate::BitReader<bool>;
#[doc = "Field `SWI36` writer - SWI36"]
pub type SWI36_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, bool, O>;
#[doc = "Field `SWI37` reader - SWI37"]
pub type SWI37_R = crate::BitReader<bool>;
#[doc = "Field `SWI37` writer - SWI37"]
pub type SWI37_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, bool, O>;
#[doc = "Field `SWI38` reader - SWI38"]
pub type SWI38_R = crate::BitReader<bool>;
#[doc = "Field `SWI38` writer - SWI38"]
pub type SWI38_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - SWI35"]
    #[inline(always)]
    pub fn swi35(&self) -> SWI35_R {
        SWI35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SWI36"]
    #[inline(always)]
    pub fn swi36(&self) -> SWI36_R {
        SWI36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SWI37"]
    #[inline(always)]
    pub fn swi37(&self) -> SWI37_R {
        SWI37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SWI38"]
    #[inline(always)]
    pub fn swi38(&self) -> SWI38_R {
        SWI38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - SWI35"]
    #[inline(always)]
    #[must_use]
    pub fn swi35(&mut self) -> SWI35_W<3> {
        SWI35_W::new(self)
    }
    #[doc = "Bit 4 - SWI36"]
    #[inline(always)]
    #[must_use]
    pub fn swi36(&mut self) -> SWI36_W<4> {
        SWI36_W::new(self)
    }
    #[doc = "Bit 5 - SWI37"]
    #[inline(always)]
    #[must_use]
    pub fn swi37(&mut self) -> SWI37_W<5> {
        SWI37_W::new(self)
    }
    #[doc = "Bit 6 - SWI38"]
    #[inline(always)]
    #[must_use]
    pub fn swi38(&mut self) -> SWI38_W<6> {
        SWI38_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](index.html) module"]
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier2::R](R) reader structure"]
impl crate::Readable for SWIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier2::W](W) writer structure"]
impl crate::Writable for SWIER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
