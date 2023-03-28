#[doc = "Register `TARG2_FN_MOD_LB` reader"]
pub struct R(crate::R<TARG2_FN_MOD_LB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARG2_FN_MOD_LB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARG2_FN_MOD_LB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARG2_FN_MOD_LB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARG2_FN_MOD_LB` writer"]
pub struct W(crate::W<TARG2_FN_MOD_LB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARG2_FN_MOD_LB_SPEC>;
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
impl From<crate::W<TARG2_FN_MOD_LB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARG2_FN_MOD_LB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FN_MOD_LB` reader - Controls burst breaking of long bursts"]
pub type FN_MOD_LB_R = crate::BitReader<bool>;
#[doc = "Field `FN_MOD_LB` writer - Controls burst breaking of long bursts"]
pub type FN_MOD_LB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TARG2_FN_MOD_LB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FN_MOD_LB_R {
        FN_MOD_LB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    #[must_use]
    pub fn fn_mod_lb(&mut self) -> FN_MOD_LB_W<0> {
        FN_MOD_LB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [targ2_fn_mod_lb](index.html) module"]
pub struct TARG2_FN_MOD_LB_SPEC;
impl crate::RegisterSpec for TARG2_FN_MOD_LB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [targ2_fn_mod_lb::R](R) reader structure"]
impl crate::Readable for TARG2_FN_MOD_LB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [targ2_fn_mod_lb::W](W) writer structure"]
impl crate::Writable for TARG2_FN_MOD_LB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TARG2_FN_MOD_LB to value 0x04"]
impl crate::Resettable for TARG2_FN_MOD_LB_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
