#[doc = "Register `DDRCTRL_DBG0` reader"]
pub struct R(crate::R<DDRCTRL_DBG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DBG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DBG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DBG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DBG0` writer"]
pub struct W(crate::W<DDRCTRL_DBG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DBG0_SPEC>;
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
impl From<crate::W<DDRCTRL_DBG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DBG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_WC` reader - DIS_WC"]
pub type DIS_WC_R = crate::BitReader<bool>;
#[doc = "Field `DIS_WC` writer - DIS_WC"]
pub type DIS_WC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_DBG0_SPEC, bool, O>;
#[doc = "Field `DIS_COLLISION_PAGE_OPT` reader - DIS_COLLISION_PAGE_OPT"]
pub type DIS_COLLISION_PAGE_OPT_R = crate::BitReader<bool>;
#[doc = "Field `DIS_COLLISION_PAGE_OPT` writer - DIS_COLLISION_PAGE_OPT"]
pub type DIS_COLLISION_PAGE_OPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DBG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DIS_WC"]
    #[inline(always)]
    pub fn dis_wc(&self) -> DIS_WC_R {
        DIS_WC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DIS_COLLISION_PAGE_OPT"]
    #[inline(always)]
    pub fn dis_collision_page_opt(&self) -> DIS_COLLISION_PAGE_OPT_R {
        DIS_COLLISION_PAGE_OPT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIS_WC"]
    #[inline(always)]
    #[must_use]
    pub fn dis_wc(&mut self) -> DIS_WC_W<0> {
        DIS_WC_W::new(self)
    }
    #[doc = "Bit 4 - DIS_COLLISION_PAGE_OPT"]
    #[inline(always)]
    #[must_use]
    pub fn dis_collision_page_opt(&mut self) -> DIS_COLLISION_PAGE_OPT_W<4> {
        DIS_COLLISION_PAGE_OPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL debug register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dbg0](index.html) module"]
pub struct DDRCTRL_DBG0_SPEC;
impl crate::RegisterSpec for DDRCTRL_DBG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dbg0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DBG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dbg0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DBG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_DBG0 to value 0"]
impl crate::Resettable for DDRCTRL_DBG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
