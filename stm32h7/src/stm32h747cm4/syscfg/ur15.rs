#[doc = "Register `UR15` reader"]
pub struct R(crate::R<UR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR15` writer"]
pub struct W(crate::W<UR15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR15_SPEC>;
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
impl From<crate::W<UR15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D2STPRST` reader - D2 Stop Reset"]
pub type D2STPRST_R = crate::BitReader<bool>;
#[doc = "Field `D2STPRST` writer - D2 Stop Reset"]
pub type D2STPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, UR15_SPEC, bool, O>;
#[doc = "Field `FZIWDGSTB` reader - Freeze independent watchdog in Standby mode"]
pub type FZIWDGSTB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - D2 Stop Reset"]
    #[inline(always)]
    pub fn d2stprst(&self) -> D2STPRST_R {
        D2STPRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Freeze independent watchdog in Standby mode"]
    #[inline(always)]
    pub fn fziwdgstb(&self) -> FZIWDGSTB_R {
        FZIWDGSTB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D2 Stop Reset"]
    #[inline(always)]
    #[must_use]
    pub fn d2stprst(&mut self) -> D2STPRST_W<0> {
        D2STPRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur15](index.html) module"]
pub struct UR15_SPEC;
impl crate::RegisterSpec for UR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur15::R](R) reader structure"]
impl crate::Readable for UR15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur15::W](W) writer structure"]
impl crate::Writable for UR15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR15 to value 0"]
impl crate::Resettable for UR15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
