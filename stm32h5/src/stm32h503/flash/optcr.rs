#[doc = "Register `OPTCR` reader"]
pub struct R(crate::R<OPTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCR` writer"]
pub struct W(crate::W<OPTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR_SPEC>;
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
impl From<crate::W<OPTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
pub type OPTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
#[doc = "Field `OPTSTRT` reader - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It’s set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It’s reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
pub type OPTSTRT_R = crate::BitReader<bool>;
#[doc = "Field `OPTSTRT` writer - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It’s set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It’s reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
pub type OPTSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
#[doc = "Field `SWAP_BANK` reader - Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR."]
pub type SWAP_BANK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It’s set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It’s reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR."]
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<0> {
        OPTLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It’s set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It’s reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<1> {
        OPTSTRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr](index.html) module"]
pub struct OPTCR_SPEC;
impl crate::RegisterSpec for OPTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optcr::R](R) reader structure"]
impl crate::Readable for OPTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optcr::W](W) writer structure"]
impl crate::Writable for OPTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTCR to value 0x01"]
impl crate::Resettable for OPTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
