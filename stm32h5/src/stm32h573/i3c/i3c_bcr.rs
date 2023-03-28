#[doc = "Register `I3C_BCR` reader"]
pub struct R(crate::R<I3C_BCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_BCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_BCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_BCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I3C_BCR` writer"]
pub struct W(crate::W<I3C_BCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_BCR_SPEC>;
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
impl From<crate::W<I3C_BCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_BCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCR0` reader - max data speed limitation"]
pub type BCR0_R = crate::BitReader<bool>;
#[doc = "Field `BCR0` writer - max data speed limitation"]
pub type BCR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_BCR_SPEC, bool, O>;
#[doc = "Field `BCR2` reader - in-band interrupt (IBI) payload"]
pub type BCR2_R = crate::BitReader<bool>;
#[doc = "Field `BCR2` writer - in-band interrupt (IBI) payload"]
pub type BCR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_BCR_SPEC, bool, O>;
#[doc = "Field `BCR6` reader - controller capable"]
pub type BCR6_R = crate::BitReader<bool>;
#[doc = "Field `BCR6` writer - controller capable"]
pub type BCR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_BCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - max data speed limitation"]
    #[inline(always)]
    pub fn bcr0(&self) -> BCR0_R {
        BCR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - in-band interrupt (IBI) payload"]
    #[inline(always)]
    pub fn bcr2(&self) -> BCR2_R {
        BCR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - controller capable"]
    #[inline(always)]
    pub fn bcr6(&self) -> BCR6_R {
        BCR6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - max data speed limitation"]
    #[inline(always)]
    #[must_use]
    pub fn bcr0(&mut self) -> BCR0_W<0> {
        BCR0_W::new(self)
    }
    #[doc = "Bit 2 - in-band interrupt (IBI) payload"]
    #[inline(always)]
    #[must_use]
    pub fn bcr2(&mut self) -> BCR2_W<2> {
        BCR2_W::new(self)
    }
    #[doc = "Bit 6 - controller capable"]
    #[inline(always)]
    #[must_use]
    pub fn bcr6(&mut self) -> BCR6_W<6> {
        BCR6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I3C bus characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i3c_bcr](index.html) module"]
pub struct I3C_BCR_SPEC;
impl crate::RegisterSpec for I3C_BCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i3c_bcr::R](R) reader structure"]
impl crate::Readable for I3C_BCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i3c_bcr::W](W) writer structure"]
impl crate::Writable for I3C_BCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I3C_BCR to value 0"]
impl crate::Resettable for I3C_BCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
